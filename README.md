# Moon-Assistant

Moon-Assistant es un **administrador** de proyectos de **Moon Engine**, es diseñado como una solucion a la administracion que requieren este tipo de proyectos.

#### Requisitos
- [Curl](https://curl.se/)
- [Git](https://git-scm.com/)
- Un compilador de C++, recomendado [gcc](https://gcc.gnu.org/) o [clang](https://clang.llvm.org/)
- [Make](https://www.gnu.org/software/make/)
- [CMake](https://cmake.org/)
- [Next](https://github.com/KEGEStudios/Next)

#### Instalación

- Linux
    ```
    curl -s https://raw.githubusercontent.com/KEGEStudios/moon-assistant/master/moon-install.sh | bash -s
    ```
    - Añade ```$HOME/opt/moon-assistant/build``` a la variable ```$PATH```
- Windows
    - Clona este repositorio
    - Compila con cmake
    - Añade al PATH el ejecutable de next

#### Comandos

- **moon create < nombre >** *Crea un nuevo proyecto con el nombre selecionado*
- **moon build** *Compila el proyecto*
- **moon run** *Ejecuta y compila si es e caso el peroyecto*
- **moon --help** o **moon -h** Muestra una guia de ayuda para el uso de **Moon-Assistant**
- **moon --version** o **moon -v** Muestra la version de **Moon** y **Moon-Assistant** que se tiene instalado

#### Contribuidores

**Moon-Assistant** es prencipalmente apollado por el equipo de desarrollo del **Game Engine MOON** creado por **KEGE Studios**.

#### Futuras verisiones

La version actual de Moon es la v1.0.0 pero el desarrollo de Moon esta en constante evolucion y se planea tener para las peroximas versiones:
- **moon upgrade** Actualización sencilla de **Next**
- **moon test** Intregracion de **Next** y Unit Testing
- **moon doctor** Una forma facil de visualizar el estado de los compiladores de C/C++
- **moon import** Conectar de forma sencilla librerias de C/C++ desde un servidor de unico de **Moon** y **Next**

#### Sobre nosotros

Los desarrolladores encargados de **Moon** en **KEGE Studios** somos un grupo de estudiantes de la Facultad de Ingenieria de la UNAM apasionados por la creación  videojuegos y programacion en C/C++
