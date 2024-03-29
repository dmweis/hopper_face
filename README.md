# Hopper Face Controller

[![codecov](https://codecov.io/gh/dmweis/hopper_face/branch/main/graph/badge.svg)](https://codecov.io/gh/dmweis/hopper_face)
[![Rust](https://github.com/dmweis/hopper_face/workflows/Rust/badge.svg)](https://github.com/dmweis/hopper_face/actions)
[![Arm cross compile](https://github.com/dmweis/hopper_face/actions/workflows/arm-cross-compile.yml/badge.svg)](https://github.com/dmweis/hopper_face/actions/workflows/arm-cross-compile.yml)
[![Security audit](https://github.com/dmweis/hopper_face/workflows/Security%20audit/badge.svg)](https://github.com/dmweis/hopper_face/actions)
[![Private docs](https://github.com/dmweis/hopper_face/workflows/Deploy%20Docs%20to%20GitHub%20Pages/badge.svg)](https://davidweis.dev/hopper_face/hopper_face/index.html)

Controller library for [Hopper's](https://github.com/dmweis/Hopper_ROS) face rewritten in rust in Rust
for use in [hopper_rust](https://github.com/dmweis/hopper_rust)

## [Click here for more images](https://davidweis.dev/robotics/2019/09/21/HopperGallery2019.html)

[![Hopper](https://github.com/dmweis/Hopper_ROS/raw/master/images/ucreate_pretty.JPG)](https://davidweis.dev/robotics/2019/09/21/HopperGallery2019.html)

## [Click here to see live CAD model!](https://davidweis.dev/robotics/2019/06/22/HopperModels.html)

[![Cad model can be viewed here](https://github.com/dmweis/Hopper_ROS/raw/master/images/hopper_cad.jpg)](https://davidweis.dev/robotics/2019/06/22/HopperModels.html)

## [Click here for videos](https://www.youtube.com/playlist?list=PL2rJqSX7Z5cFj5UM5ozf1wcm_McQg75ch)

[![Hopper climbing obstacle](https://img.youtube.com/vi/faWG_BYd5a0/0.jpg)](https://www.youtube.com/playlist?list=PL2rJqSX7Z5cFj5UM5ozf1wcm_McQg75ch)

Hopper (Named after Grace Hopper) is a 3D printed hexapod robot running on ROS.  
It's brain is a Raspberry Pi 3 running ~~Ubunut Xenial with ROS Kinetic~~ Slowly porting into pure Rust.  
The platform is modeled and 3D printed by me and is still a work in progress. It's heavily inspired by [PhantomX Hexapod from Trossen robotics](http://www.trossenrobotics.com/phantomx-ax-hexapod.aspx)

CAD design files can be found [here](https://github.com/dmweis/hopper_design)  
They may be outdated. If you'd like current file you can email me [here](mailto:dweis7@gmail.com)

## Cross compilation

To cross compile for raspberry pi you need to add `armv7-unknown-linux-musleabihf` target to rustup

You will also need an arm compatible compiler. On debian that is `gcc-arm-linux-gnueabihf`

To set this up run:

```shell
rustup target add armv7-unknown-linux-musleabihf
rustup target add aarch64-unknown-linux-musl
sudo apt update
sudo apt install gcc-arm-linux-gnueabihf gcc-aarch64-linux-gnu
```

Then compile for arm with `cargo build --target=armv7-unknown-linux-musleabihf`

The [deploy script](./deploy) and [cross_build script](./cross_build) sort of do it for you.
