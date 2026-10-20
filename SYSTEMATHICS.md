# Systemathics fork

Upstream: https://github.com/nazarii-piontko/datafusion-sharp. This branch, `systemathics`, is based on upstream tag `v0.6.3`.

## Differences from upstream

- The native library allocates through mimalloc on every OS (`native/src/lib.rs`, `#[global_allocator]`). Windows' default heap hands large blocks (more than about 512 KB, such as the column buffers of 64K-row batches) to `VirtualAlloc`/`VirtualFree` on every allocation. glibc keeps and reuses that memory, so the default allocators give different performance on Windows and Linux. mimalloc keeps and reuses it on both.
- The packages are `Systemathics.DataFusionSharp` and `Systemathics.DataFusionSharp.Data`. Their own ids keep them apart from upstream's packages in a shared NuGet cache. The namespaces and the assembly names are unchanged.
- No macOS package: nobody runs it on a Mac, and GitHub's macOS arm64 runners were too scarce to pick up the build. The native libraries are linux-x64, linux-arm64, win-x64 and linux-musl-x64 (Alpine, which upstream does not ship: its glibc library needs ld-linux-x86-64.so.2, which Alpine has not), built in an Alpine container with the C runtime linked dynamically (native/.cargo/config.toml).
- Nothing leaves GitHub Actions. The release workflow publishes no package and creates no GitHub release: the packages stay a run artifact, uploaded by hand to `systemathics-internal` on Nexus. The SonarCloud job is gone (it needs upstream's token), as are the docs deployment to GitHub Pages and Dependabot.

## Releasing

1. Tag the commit `v<upstream version>-mimalloc.<n>`, for example `v0.6.3-mimalloc.1`, and push the tag. MinVer takes the package version from the tag.
2. The `Release` workflow builds the four native libraries (linux-x64, linux-arm64, win-x64, linux-musl-x64), and packs. There is no package smoke test: it builds an app against the packages by id, which only resolves from a feed they are published to.
3. Download the `nuget-package` artifact from the run's summary page (kept 30 days), and upload its `.nupkg`/`.snupkg` to https://nexus3.systemathics.io:8443/repository/systemathics-internal.

## Rebasing on a newer upstream

Rebase `systemathics` onto the new upstream tag, then release as `v<new version>-mimalloc.1`.
