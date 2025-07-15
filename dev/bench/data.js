window.BENCHMARK_DATA = {
  "lastUpdate": 1752546388283,
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
      }
    ]
  }
}