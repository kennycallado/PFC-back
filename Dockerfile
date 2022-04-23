## IMPORTANTE -> pasos previos v- abajo
#
# crear imágen
# podman build --no-cache --platform linux/arm64 -t kennycallado/sensacion_api:v4-slim-arm64 .

# FROM alpine:latest
FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/sensacion /bin/sensacion
# COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/sensacion /bin/sensacion
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "sensacion" ]

### IMPORTANTE:
#
# Creo que será mejor antes compilar y después copiar el binario...
# docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder
# rust-musl-builder cargo build --release
#
# Creo que también hay un problema con los permisos al usar podman... pero bueno
# pueden cambiarse desde el contenedor y luego al volver pasarlos a kenny
#
# -------
#
# OJO: en Rocket.toml hay que modificar la url... x alguna razón no me coge
# las variables de entorno... tengo que estudiarlo
#
# 
