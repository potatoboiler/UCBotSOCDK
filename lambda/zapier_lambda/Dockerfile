# noop
FROM rust:latest as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /zapier_lambda

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /zapier_lambda

# Copy our build
COPY --from=builder /zapier_lambda/target/x86_64-unknown-linux-musl/release/zapier_lambda ./

# Use an unprivileged user.
USER zapier_lambda:zapier_lambda

CMD ["/zapier_lambda/zapier_lambda"]