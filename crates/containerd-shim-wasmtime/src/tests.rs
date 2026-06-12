use std::time::Duration;

use containerd_shim_wasm::testing::modules::*;
use containerd_shim_wasm::testing::{WasiTest, oci_helpers};
use serial_test::serial;
use wasmtime::{Config, Engine};

use crate::WasmtimeShim as WasiEngine;

#[test]
#[serial]
fn test_delete_after_create() -> anyhow::Result<()> {
    WasiTest::<WasiEngine>::builder()?.build()?.delete()?;
    Ok(())
}

#[test]
#[serial]
fn test_hello_world() -> anyhow::Result<()> {
    let (exit_code, stdout, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_hello_world_oci() -> anyhow::Result<()> {
    let (builder, _oci_cleanup) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(None, None)?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_hello_world_oci_uses_precompiled() -> anyhow::Result<()> {
    let (builder, _oci_cleanup1) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c1".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    let (label, _id) = oci_helpers::get_content_label()?;
    assert!(
        label.starts_with("runwasi.io/precompiled/wasmtime/"),
        "was {}",
        label
    );

    // run second time, it should succeed without recompiling
    let (builder, _oci_cleanup2) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c2".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_hello_world_oci_uses_precompiled_when_content_removed() -> anyhow::Result<()> {
    let (builder, _oci_cleanup1) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c1".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    // remove the compiled content from the cache
    let (label, id) = oci_helpers::get_content_label()?;
    assert!(
        label.starts_with("runwasi.io/precompiled/wasmtime/"),
        "was {}",
        label
    );
    oci_helpers::remove_content(id)?;

    // run second time, it should succeed
    let (builder, _oci_cleanup2) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c2".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_custom_entrypoint() -> anyhow::Result<()> {
    let (exit_code, stdout, _) = WasiTest::<WasiEngine>::builder()?
        .with_start_fn("foo")
        .with_wasm(CUSTOM_ENTRYPOINT)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_unreachable() -> anyhow::Result<()> {
    let (exit_code, _, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(UNREACHABLE)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_ne!(exit_code, 0);

    Ok(())
}

#[test]
#[serial]
fn test_exit_code() -> anyhow::Result<()> {
    let (exit_code, _, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(EXIT_CODE)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 42);

    Ok(())
}

#[test]
#[serial]
fn test_seccomp() -> anyhow::Result<()> {
    let (exit_code, stdout, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(SECCOMP)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "current working dir: /");

    Ok(())
}

#[test]
#[serial]
fn test_has_default_devices() -> anyhow::Result<()> {
    let (exit_code, _, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HAS_DEFAULT_DEVICES)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);

    Ok(())
}

// Test that the shim can execute an named exported function
// that is not the default _start function in a wasm component.
// The current limitation is that there is no way to pass arguments
// to the exported function.
// Issue that tracks this: https://github.com/containerd/runwasi/issues/414
#[test]
#[serial]
fn test_simple_component() -> anyhow::Result<()> {
    let (exit_code, _, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(SIMPLE_COMPONENT)?
        .with_start_fn("thunk")
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);

    Ok(())
}

// Test that the shim can execute a wasm component that is
// compiled with wasip2.
//
// This is using the `wasi:cli/command` world to run the component.
//
// The wasm component is built and copied over from
// https://github.com/Mossaka/wasm-component-hello-world. See
// README.md for how to build the component.
#[test]
#[serial]
fn test_wasip2_component() -> anyhow::Result<()> {
    let (exit_code, stdout, _) = WasiTest::<WasiEngine>::builder()?
        .with_wasm(COMPONENT_HELLO_WORLD)?
        .build()?
        .start()?
        .wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "Hello, world!\n");

    Ok(())
}

// Test that the shim can execute a wasm component that is
// compiled with wasi:http/proxy.
//
// This is using the `wasi:http/proxy` world to run the component.
//
// The wasm component is built using cargo component as illustrated in the following example::
// https://opensource.microsoft.com/blog/2024/09/25/distributing-webassembly-components-using-oci-registries/
#[test]
#[serial]
fn test_wasip2_component_http_proxy() -> anyhow::Result<()> {
    let srv = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WASI_HTTP)?
        .with_host_network()
        .build()?;

    let srv = srv.start()?;
    let response = http_get();

    let response = response.expect("Server did not start in time");
    assert!(response.status().is_success());

    let body = response.text().unwrap();
    assert_eq!(body, "Hello, this is your first wasi:http/proxy world!\n");

    let (exit_code, _, _) = srv.ctrl_c()?.wait(Duration::from_secs(5))?;
    assert_eq!(exit_code, 0);

    Ok(())
}

// The wasm component is built using componentize-dotnet as illustrated in the following example::
// https://bytecodealliance.org/articles/simplifying-components-for-dotnet-developers-with-componentize-dotnet
// this ensures we are able to use wasm built from other languages https://github.com/containerd/runwasi/pull/723
#[test]
#[serial]
fn test_wasip2_component_http_proxy_csharp() -> anyhow::Result<()> {
    let srv = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WASI_HTTP_CSHARP)?
        .with_host_network()
        .build()?;

    let srv = srv.start()?;

    // dotnet takes a bit longer to start up
    // Todo: find out why this doesn't happen in wasmtime directly
    let response = http_get_with_backoff_secs(2);

    let response = response.expect("Server did not start in time");
    assert!(response.status().is_success());

    let body = response.text().unwrap();
    assert_eq!(body, "Hello, from C#!");

    let (exit_code, _, _) = srv.ctrl_c()?.wait(Duration::from_secs(5))?;
    assert_eq!(exit_code, 0);

    Ok(())
}

// Test that the shim can terminate component targeting wasi:http/proxy by sending SIGTERM.
#[test]
#[serial]
fn test_wasip2_component_http_proxy_force_shutdown() -> anyhow::Result<()> {
    let srv = WasiTest::<WasiEngine>::builder()?
        .with_wasm(HELLO_WASI_HTTP)?
        .with_host_network()
        .build()?;

    let srv = srv.start()?;
    assert!(http_get().unwrap().status().is_success());

    // Send SIGTERM
    let (exit_code, _, _) = srv.terminate()?.wait(Duration::from_secs(5))?;
    // The exit code indicates that the process did not exit cleanly
    assert_eq!(exit_code, 128 + libc::SIGTERM as u32);

    Ok(())
}

// Test that an OCI image containing precompiled (serialized) wasmtime bytes
// is rejected rather than being unsafely deserialized.
//
// This is the core security property: only layers fetched from the containerd
// cache (marked as precompiled) should be deserialized; layers from OCI images
// should never be treated as precompiled even if their bytes match the format.
#[test]
#[serial]
fn test_oci_image_with_precompiled_bytes_is_rejected() -> anyhow::Result<()> {
    // Precompile the hello world wasm module using the same engine config
    // as the shim to produce valid precompiled bytes.
    let mut config = Config::new();
    config.parallel_compilation(false);
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let precompiled_bytes = engine.precompile_module(HELLO_WORLD.as_ref())?;

    // Verify the bytes are actually detected as precompiled
    assert!(
        Engine::detect_precompiled(&precompiled_bytes).is_some(),
        "generated bytes should be detected as precompiled"
    );

    // Import an OCI image using the precompiled bytes as the wasm layer.
    // These bytes are invalid as OCI layer content since precompiled modules
    // should only be loaded from the containerd cache, not from image layers.
    let image_name = "localhost/invalid-precompiled:latest".to_string();
    let precompiled_content = oci_helpers::ImageContent {
        bytes: precompiled_bytes,
        media_type: "application/vnd.bytecodealliance.wasm.component.layer.v0+wasm".to_string(),
    };
    oci_helpers::import_image(&image_name, &[&precompiled_content])?;

    // Use as_oci_image with the pre-imported invalid image.
    // Since the image already exists, as_oci_image will skip re-importing and
    // just create a container pointing to it. It also removes the local wasm
    // file from rootfs so the runtime uses the OCI layer path.
    let (builder, _oci_cleanup) = WasiTest::<WasiEngine>::builder()?
        .as_oci_image(Some(image_name), Some("invalid-precompiled".to_string()))?;

    let test = builder.build()?;
    let result = test.start();

    // The container should fail: either start() returns an error,
    // or if it starts, wait() should return a non-zero exit code.
    let test = match result {
        Ok(test) => test,
        Err(err) => {
            println!("failed to start container as expected: {err}");
            return Ok(());
        }
    };

    // If it started successfully, wait for it to exit and check the exit code.
    let (exit_code, stdout, _) = test.wait(Duration::from_secs(10))?;

    println!("container started with exit code {exit_code} and stdout: {stdout:?}");

    assert_ne!(
        stdout, "hello world\n",
        "precompiled module from OCI layer should not have been unsafely deserialized"
    );
    assert_ne!(
        exit_code, 0,
        "container should not succeed via precompiled deserialization"
    );

    Ok(())
}

fn http_get() -> reqwest::Result<reqwest::blocking::Response> {
    http_get_with_backoff_secs(1)
}

// Helper method to make a `GET` request
fn http_get_with_backoff_secs(backoff: u64) -> reqwest::Result<reqwest::blocking::Response> {
    const MAX_ATTEMPTS: u32 = 10;
    let backoff_duration: Duration = Duration::from_secs(backoff);

    let mut attempts = 0;

    loop {
        match reqwest::blocking::get("http://127.0.0.1:8080") {
            Ok(resp) => break Ok(resp),
            Err(err) if attempts == MAX_ATTEMPTS => break Err(err),
            Err(_) => {
                std::thread::sleep(backoff_duration);
                attempts += 1;
            }
        }
    }
}
