# PFC - Sensación - server

Este repositorio es parte del proyecto final de ciclo.

## Descripción

Este módulo del proyecto pretende servir de intermediario entre el módulo de font-end y la base de datos. Se entiende como _api_ de la aplicación, definiendo una serie de reglas de comunicación y trasmitiendo la información en formato _json_.

He procurado seguir el patrón MVC [link](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller) con repositorio para organizar el código. Los ficheros de configuración del proyecto residen en el directorio raíz, así como las migraciones. Dentro del directorio _src_ está la aplicación en sí. Contiene el fichero _main.rs_ desde donde se inicia la ejecución, el fichero _server.rs_ que agrega la configuración e inicia el servidor http y el fichero _routes.rs_ que especifica las rutas cubiertas por el servidor.

El directorio _config_ recibe la configuración propia de la aplicación, como la base de datos que posteriormente es usada en _server.rs_.

Finalmente el directorio _app_ que contiene la lógica de la aplicación distribuida en los directorios _controllers_, _models_ y repositories_. Cada uno cumple las funciones determinadas del patrón de diseño.

Los ficheros _mod.rs_ son usados por _Rust_ para hacer público los módulos contenidos en el directorio.

### Tecnologías

- Rust
- Rocket
- Diesel
- Postgres
- Docker

## Instalación

Voy a centrarme en el proceso de compilación del proyecto y la configuración necesaria para que el binario resultante funcione correctamente. Existiría otro enfoque dirigido al desarrollo de la aplicación que podría explicarse en una sección específica para desarrolladores.

Para los pasos descritos se entiende que se usará un sistema operativo linux. Podría usarse windows pero se escapa del alcance de la guía.

### Dependencias

- rust
- cargo
- diesel cli (opcional)
- libpq
- postgres

Para compilar el proyecto primero se debe tener instalado el lenguaje [link](https://www.rust-lang.org/tools/install), así como su gestor de paquetes _Cargo_. También es necesario tener instalada la librería _libpq_ ([link](https://packages.debian.org/bullseye/libpq-dev)) ya que el orm (diesel) usa código nativo para comunicarse con la base de datos. Opcionalmente se puede tener instalado el cli de este orm [link](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) para ejecutar las migraciones, pero también pueden ejecutarse manualmente una vez iniciado el servidor de postgres. Por último, es necesaria una base de datos, para este proyecto he usado el gestor de base de datos postgres. Esta última dependencia también puede satisfacerse a través de un contenedor, por ejemplo, con docker-compose, y para ello incluyo el fichero _compose.yml_.

### Lenguaje

Para instalarlo fácilmente se puede ejecutar el siguiente comando y escoger la opción 1:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Libpq

En debian se puede instalar con el siguiente comando:

``` bash
sudo apt install libpq5 libpq-dev
```

### Base de datos

#### Docker

En caso de usar _docker-compose_ solo debe ejecutarse el siguiente comando:

``` bash
docker-compose up -d
```

 Para detener el contenedor debe ejecutarse el siguiente comando:

 ``` bash
docker-compose down
 ```

 Este manifiesto (_compose.yml_) está configurado para coincidir con las credenciales de conexión que usa el proyecto por defecto. En caso de no usar este método debe tenerse postgres instalado y tener en cuenta el siguiente punto.

#### Postgres

Las credenciales de conexión con la base de datos deben configurarse en el fichero _Rocket.toml_, concretamente en el apartado _[default.databases.diesel]_. En este apartado debe especificarse el campo _url_ con la ruta de conexión. En el siguiente ejemplo deben sustituirse el contenido entre llaves por los valores correctos.

``` toml
[default.databases.diesel]
url = "postgres://{user}:{password}@{host}/{db}"
```

Para completar la base de datos con las tablas requeridas debe ejecutarse el siguiente script hacia la base de datos creada:

``` sql
CREATE TABLE IF NOT EXISTS tables (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  max_people INTEGER NOT NULL,
  min_people INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS bookings (
  id SERIAL PRIMARY KEY,
  tables_id SERIAL,
  username VARCHAR NOT NULL,
  people INTEGER NOT NULL,
  date_book VARCHAR NOT NULL,
  CONSTRAINT fk_book_table
    FOREIGN KEY(tables_id) REFERENCES tables(id)
);
```

#### Migraciones

En caso de haber instalado el cli de diesel, debe existir una variable de entorno con la url de conexión _DATABASE\_URL_ o un fichero _.env_ con esta misma información.

``` bash
DATABASE_URL=postgres://{user}:{password}@{host}/{db}
```

Con esto solo resta ejecutar las migraciones con el siguiente comando:

``` bash
diesel migration run
```

Este comando puebla la base de datos con las tablas necesarias, así como una función de diesel que le permite seguir las migraciones ejecutadas.

### Compilación

Para compilar el proyecto debe ejecutarse el siguiente comando que se encargará de resolver las dependencias, compilarlas y finalmente compilar el binario de la aplicación:

``` bash
cargo build --release
```

### Ejecución

Ahora para ejecutar la aplicación debe hacerse desde la ubicación donde esté el fichero _Rocket.toml_ de donde se tomarán la url de conexión con la base de datos. Por ejemplo, estando en la raíz del proyecto (donde reside dicho fichero) puede ejecutarse:

```
./target/release/sensacion
```

Esto inicia el servidor y queda a la escucha de peticiones y si no se han realizado modificaciones en el fichero de configuración escuchará en el puerto _8000_. Se muestra en consola las rutas y otros datos importantes.

En caso de no mostrarse nada el problema suele ser que no se ha iniciado la base de datos.

## Imagen para producción

Debido a que se pretende desplegar la aplicación contenerizada sobre servidores kubernetes, deben tenerse en cuenta algunos puntos importantes. Por ejemplo, la imagen base que se usará para economizar recursos y la arquitectura del servidor.

He procurado usar una imagen base lo más liviana posible para que la imagen resultante ocupe el mínimo posible de recursos en el servidor. Para ello usaré _busybox_ [Link](https://hub.docker.com/_/busybox) (imagen de 1M) que es incluso más ligera que _alpine_ [Link](https://hub.docker.com/_/alpine) (imagen de 5M). Busybox nos proporciona todo lo que necesitamos para ejecutar un binario en todas las arquitecturas que existen, con el único inconveniente que no dispone de _glibc_ [Link](https://en.wikipedia.org/wiki/Glibc) librearía que por ejemplo _libpq_ necesita. Para resolver este problema compilaré el proyecto desde una imagen de docker (ekidd/rust-musl-builder) que resuelve estas dependencias usando musl [Link](https://musl.libc.org/), alternativa a glibc que usan _busybox_ y _alpine_.

Por otro lado, debe notarse que se pretende desplegar sobre una raspberry pi 4. Esta placa usa un procesador _ARM_ por lo que la imagen a distribuir debe seguir una arquitectura _arm64_. Para conseguir esto ha de generarse la imagen de docker para esta arquitectura.

### Dependencias

Debe tenerse instalado _docker_ o _podman_ en el sistema. Para docker es necesario activar _buildx_ (en versiones recientes activo por defecto) que permite construir imágenes en diferentes arquitecturas, con podman debe instalarse la librería _qemu-user-static_ para disponer de esta misma funcionalidad.

En los siguientes pasos describo el proceso usando _podman_ pero sería muy parecido usando _docker_.

NOTA: debido a que por alguna razón las credenciales de conexión con la base de datos no las toma desde variables de entorno, debe configurarse previamente el fichero _Rocket.toml_ para el despliegue. Actualmente esto supone comentar la línea de _url_ actual y descomentar la siguiente línea.

### Proceso

Desde la raíz del proyecto ejecutar el siguiente comando:

``` bash
docker run --rm -it -v $(pwd):/home/rust/src ekidd/rust-musl-builder
```
Este comando iniciará un contenedor en modo interactivo con un sistema preparado para compilar el proyecto con _musl_. Por seguridad se ejecuta sin privilegios de administrador, por lo que en este punto, podría concederse permisos para que pueda guardar la compilación.

``` bash
sudo chmod -R o+w target
```

Ahora se puede compilar desde el contenedor con el siguiente comando:

```
cargo build --release
```

Al terminar se puede salir del contenedor `exit`. El resultado de la compilación quedará en un directorio dentro de target.

El siguiente paso es generar la imagen de docker a partir de la compilación realizada. El fichero _Dockerfile_ está preparado para ello por lo que solo se debe ejecutar el siguiente comando:

``` bash
podman build --no-cache --pull --platform linux/arm64 -t kennycallado/sensacion_api:vX-slim-arm64 .
```

Nótese que he usado un tag relativo a mi cuenta en https://hub.docker.com.

### Registro remoto

Para que el resultado de todos estos pasos previos sea accesible desde cualquier punto, usaré el registro de imágenes oficial de docker, y para ello debe ejecutar el siguiente comando:

``` bash
podman push --format docker kennycallado/sensacion_api:vX-slim-arm64
```

Para que funcione previamente ha de realizar el login con las credenciales oportunas:

``` bash
podman login
```

### Despliegue

Para el despliegue en kubernetes deben generarse los manifiestos necesarios que se aportan en otro repositorio (PFC-root).

## Desarrollo

### IMPORTANTE

El siguiente enlace sigue un enfoque muy parecido, pero algunas cuestiones como la piscina de peticiones a la base de datos ya está resuelta por el framework: https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104

### GIT

``` bash
git push origin HEAD:master
```

## TODO:

- [ ] quizá integrar migraciones al iniciar la base de datos.
- [ ] debería añadir if not exists al crear las tablas...
