# STM32 + LVGL

This is a template project for the STM32G4XX and LVGL. It currently does not work. To manage the build environment, I have used the arm32v7 Rust Docker container. 

## Building and running the container
Run `make docker-build` to build the container and run the container indefinitely. Subsequent runs can use `make docker`. The container is meant to stay open in the background and interacted with using the shell environment while the bugs are worked out of the build pipeline. To access the shell, you can use the Docker Desktop tool. 

## Building the project
From the Docker shell, check you are in the `/app` folder (should be default). Then run `make build` and this should run `RUST_BACKTRACE=full cargo build -Zfeatures=build_dep --target thumbv7em-none-eabihf`; it takes a bit of time to compile, unfortunately. Sometimes, `make build` will fail with a `cannot open file` error, in which case run it again. It should then fail with:

```
  = note: rust-lld: error: section '.bss' will not fit in region 'RAM': overflowed by 967936 bytes
          rust-lld: error: section '.uninit' will not fit in region 'RAM': overflowed by 967936 bytes
```

