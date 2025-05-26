# Dockerizacion


![image](https://www.ionos.mx/digitalguide/fileadmin/DigitalGuide/Screenshots_2023/docker-run-whalesay-boo-english.png)


Docker es una tecnología de código abierto que permite el libre despliegue de aplicaciones, así como todo lo relacionado con ellas, en contenedores software sin importar el sistema operativo de la máquina que se esté utilizando. Los contenedores permiten crear, implementar, ejecutar, copiar y trasladar aplicaciones con facilidad. Estas aplicaciones se pueden obtener o compartir bajo el nombre de imagen. 

En este rama se realizo la dockerizacion del proyecto y la ejecucion del entorno unicamente para el back, dockerizar se refiere a la implementación de Docker para empaquetar una aplicación (software), para luego distribuirla y ejecutarla a través de los contenedores. También se le conoce como contenerizar aplicacionesDockerizar se refiere a la implementación de Docker para empaquetar una aplicación (software), para luego distribuirla y ejecutarla a través de los contenedores. También se le conoce como contenerizar aplicaciones


## Dockerfile
```
FROM rust:latest as build
WORKDIR /usr/src/FAGIA
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=build /usr/src/FAGIA/target/release/FAGIA /usr/local/bin/FAGIA
CMD ["FAGIA"]
```

Primero se usa la imagen mas actual de rust y le pone de apodo `build` seguido de eso establece el repositorio de trabajo en `/usr/src/FAGIA`  y copia todo los archivos de la maquina a la imagen que se va a crear (excepto por los archivos que se especifican en el dockerignore)

Despues con el comando `cargo install --path .` se generan las binarias, es decir el ejecutable del proyecto en la ruta actual (directorio de trabajo).

Finalmente se utiliza la ultima imagen disponible de ubuntu para tomar de base del contenedor, despues se copia desde la instancia del contenedor que se genero con la imagen de rust la binaria resultante del programa en las binarias de ubuntu, finalmente se establece el comando por defecto que se ejecutara cuando se hagan los contenedores, siendo este FAGIA el cual es la binaria del programa generado.

[docker pull robzun/fagia](https://hub.docker.com/r/robzun/fagia)


## docker-mysql.yaml
```

version: '3.8'
services:
  db:
    image: mysql:latest
    container_name: db
    networks:
      - mi-red
    environment:
      MYSQL_ROOT_PASSWORD: root
      MYSQL_DATABASE: fagia

  fagia-app:
    image: sebatihm/fagia-app:latest
    container_name: fagia-app
    networks:
      - mi-red
    ports:
      - "8080:8080"
    depends_on:
      - db

networks:
  mi-red:
```
A pesar de haber hecho la Dockerizacion utilizamos la imagen de mi compañero sebastian, ya que tenia unos problemas con mi computadora.


Este es el docker-compose el cual se usa para levantar los servicios de Fagia, primero se crea una red llamada mi-red (aunque la crea por defecto lo configure por cualquier cosa), el compose se compone de dos servicios, fagia-app la cual es la imagen que se ha creado anteriormente, configurando que tome la red creada del docjer y exponiendo el puerto 8080 y especificando que depende del contenedor db.

El otro servico es db el cual toma de base la ultima imagen de mysql y la configura de la siguiente forma
- Usuario: root
- Contraseña: root
- Base de Datos: fagia

Despues se establece que este en la misma red que fagia (mi-red), notese que este servicio se levantara primero para que la instancia de la imagen que se creo se pueda conectar a la base de datos y funcionar correctamente.

### Diferencias
Se tuvieron que modificar ciertos aspectos de la aplicacion, las variables de entorno para que funcione correctamente, ya que al estar dockerizada se escribieron estas mismas en el codigo:
- [Address: 0.0.0.0 ] Para que tome la direccion del contenedor y que se pueda realizar el despliegue , se pone directamente en el constructor del servicio
- [Port: 8080] No sufrio cambios, se pone directamente en el constructor del servicio
- [Secret ] Se especifico en el archivo de JWT en utils la clave secreta mediante una string
- [Database_url: mysql://root:root@db:3306/fagia ] se usan las variables del contenedor especificado en el docker-compose para que este tome la base de datos del contenedor bd, se pone directamente al momento de instanciar la conexion a la base de datos.
