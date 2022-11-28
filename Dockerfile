FROM ubuntu:jammy
COPY target/release/jotsy /usr/local/bin
RUN mkdir -p /var/lib/jotsy
WORKDIR /var/lib/jotsy
CMD ["jotsy"]
EXPOSE 2022/tcp