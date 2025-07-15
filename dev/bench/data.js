window.BENCHMARK_DATA = {
  "lastUpdate": 1752546623279,
  "repoUrl": "https://github.com/cuisongliu/runwasi",
  "entries": {
    "Criterion.rs Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "dd3404f0ba9c7df9501cb4b4429cd043fbfc446b",
          "message": "Merge pull request #1015 from containerd/dependabot/cargo/patch-dd3b444903\n\nchore(deps): bump libc from 0.2.173 to 0.2.174 in the patch group",
          "timestamp": "2025-06-24T04:52:28Z",
          "url": "https://github.com/cuisongliu/runwasi/commit/dd3404f0ba9c7df9501cb4b4429cd043fbfc446b"
        },
        "date": 1752546387858,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "wamr/memory-usage",
            "value": 14804,
            "unit": "kB",
            "extra": "shim: 11984 kB\nzygote: 2820 kB"
          },
          {
            "name": "wasmedge/memory-usage",
            "value": 65692,
            "unit": "kB",
            "extra": "shim: 53004 kB\nzygote: 12688 kB"
          },
          {
            "name": "wasmer/memory-usage",
            "value": 18976,
            "unit": "kB",
            "extra": "shim: 15372 kB\nzygote: 3604 kB"
          },
          {
            "name": "wasmtime/memory-usage",
            "value": 20008,
            "unit": "kB",
            "extra": "shim: 16408 kB\nzygote: 3600 kB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "dd3404f0ba9c7df9501cb4b4429cd043fbfc446b",
          "message": "Merge pull request #1015 from containerd/dependabot/cargo/patch-dd3b444903\n\nchore(deps): bump libc from 0.2.173 to 0.2.174 in the patch group",
          "timestamp": "2025-06-24T04:52:28Z",
          "url": "https://github.com/cuisongliu/runwasi/commit/dd3404f0ba9c7df9501cb4b4429cd043fbfc446b"
        },
        "date": 1752546524707,
        "tool": "cargo",
        "benches": [
          {
            "name": "end-to-end/wasmtime/wasi-demo-app:latest",
            "value": 142837904,
            "range": "± 4119278",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmtime/wasi-demo-oci:latest",
            "value": 82588556,
            "range": "± 945210",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmedge/wasi-demo-app:latest",
            "value": 114745205,
            "range": "± 1950457",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmedge/wasi-demo-oci:latest",
            "value": 122731958,
            "range": "± 1756533",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmer/wasi-demo-app:latest",
            "value": 134651589,
            "range": "± 2464718",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wasmer/wasi-demo-oci:latest",
            "value": 136628041,
            "range": "± 3235287",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wamr/wasi-demo-app:latest",
            "value": 88077214,
            "range": "± 2335597",
            "unit": "ns/iter"
          },
          {
            "name": "end-to-end/wamr/wasi-demo-oci:latest",
            "value": 86571683,
            "range": "± 1117726",
            "unit": "ns/iter"
          }
        ]
      }
    ],
    "HTTP Throughput": [
      {
        "commit": {
          "author": {
            "name": "Jiaxiao Zhou",
            "username": "Mossaka",
            "email": "duibao55328@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "dd3404f0ba9c7df9501cb4b4429cd043fbfc446b",
          "message": "Merge pull request #1015 from containerd/dependabot/cargo/patch-dd3b444903\n\nchore(deps): bump libc from 0.2.173 to 0.2.174 in the patch group",
          "timestamp": "2025-06-24T04:52:28Z",
          "url": "https://github.com/cuisongliu/runwasi/commit/dd3404f0ba9c7df9501cb4b4429cd043fbfc446b"
        },
        "date": 1752546622794,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "HTTP RPS",
            "value": 20650.8416,
            "unit": "req/s"
          }
        ]
      }
    ]
  }
}