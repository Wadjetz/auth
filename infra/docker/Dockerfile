FROM ubuntu:20.04

RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev libpq-dev

COPY bauth /bauth/bauth

WORKDIR /

EXPOSE 5000

CMD /bauth/bauth
