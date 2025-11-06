FROM ubuntu:24.04 AS build

ENV DEBIAN_FRONTEND=noninteractive
ENV GRAAL_VERSION=21.0.2

RUN apt-get update && \
    apt-get install -y wget curl zip unzip tar build-essential musl musl-dev musl-tools maven

WORKDIR /tmp/zlib
RUN wget https://zlib.net/zlib-1.3.1.tar.gz && \
    tar -xzf zlib-1.3.1.tar.gz

WORKDIR /tmp/zlib/zlib-1.3.1
RUN CC=musl-gcc ./configure --static && \
    make && \
    mkdir -p /usr/lib/x86_64-linux-musl && \
    cp libz.a /usr/lib/x86_64-linux-musl/

WORKDIR /workspace
RUN wget https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-${GRAAL_VERSION}/graalvm-community-jdk-${GRAAL_VERSION}_linux-x64_bin.tar.gz && \
    tar -xzf graalvm-community-jdk-${GRAAL_VERSION}_linux-x64_bin.tar.gz && \
    mv graalvm-community-openjdk-${GRAAL_VERSION}* /opt/graalvm
ENV PATH="/opt/graalvm/bin:${PATH}"

WORKDIR /workspace

COPY . .
RUN ./mvnw clean package -Dpackaging=native-image

FROM scratch
WORKDIR /app

COPY --from=build /workspace/target/micronaut21-native /app/micronaut21-native
EXPOSE 8080

ENTRYPOINT ["/app/micronaut21-native"]