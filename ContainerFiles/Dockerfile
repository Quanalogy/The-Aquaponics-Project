FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.0

ENV SYSROOT=''
ENV PKG_CONFIG_DIR=''
ENV PKG_CONFIG_LIBDIR=${SYSROOT}/usr/lib/pkgconfig:${SYSROOT}/usr/share/pkgconfig
ENV PKG_CONFIG_SYSROOT_DIR=${SYSROOT}
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig

RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get install -qq pkg-config libudev-dev:armhf curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN /root/.cargo/bin/rustup target add arm-unknown-linux-gnueabihf

RUN echo '\nexport PATH=$PATH:/root/.cargo/bin\n' >> /root/.bashrc

