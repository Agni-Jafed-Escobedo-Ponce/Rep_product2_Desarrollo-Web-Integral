# NumberConversion – Cliente SOAP Multilenguaje

Repositorio público que implementa tres versiones de conversión, de número a letras en **8 lenguajes de servidor**: Ruby, Perl, Node.js, .NET 10, Golang, Java, C++ y Rust.

## Descripción de versiones

| Archivo | Descripción |
|---------|-------------|
| `clisoap1.*` | Consume el servicio SOAP público y retorna el número en **inglés** |
| `clisoap2.*` | Consume el servicio SOAP y **traduce al español** con una librería |
| `conintl.*`  | Convierte el número a letras en español usando el **propio lenguaje** |

## Servicio SOAP utilizado

```
https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL
```

Operación: `NumberToWords(ubiNum)` → devuelve el número en palabras en inglés.

## Estructura del repositorio

```
/
├── ruby/
│   ├── clisoap1.rb
│   ├── clisoap2.rb
│   └── conintl.rb
├── perl/
│   ├── clisoap1.pl
│   ├── clisoap2.pl
│   └── conintl.pl
├── nodejs/
│   ├── clisoap1.js
│   ├── clisoap2.js
│   └── conintl.js
├── dotnet/
│   ├── clisoap1/Program.cs
│   ├── clisoap2/Program.cs
│   └── conintl/Program.cs
├── golang/
│   ├── clisoap1/main.go
│   ├── clisoap2/main.go
│   └── conintl/main.go
├── java/
│   ├── clisoap1/src/main/java/com/example/Clisoap1Application.java
│   ├── clisoap2/src/main/java/com/example/Clisoap2Application.java
│   └── conintl/src/main/java/com/example/ConintlApplication.java
├── cpp/
│   ├── clisoap1/main.cpp
│   ├── clisoap2/main.cpp
│   └── conintl/main.cpp
└── rust/
    ├── clisoap1/src/main.rs
    ├── clisoap2/src/main.rs
    └── conintl/src/main.rs
```

## Puertos utilizados

| Lenguaje | clisoap1 | clisoap2 | conintl |
|----------|----------|----------|---------|
| Ruby | 3001 | 3002 | 3003 |
| Perl | 4001 | 4002 | 4003 |
| Node.js | 3001 | 3002 | 3003 |
| .NET 10 | 5001 | 5002 | 5003 |
| Golang | 8001 | 8002 | 8003 |
| Java | 8081 | 8082 | 8083 |
| C++ | 7001 | 7002 | 7003 |
| Rust | 6001 | 6002 | 6003 |

---

## Instalación y ejecución por lenguaje

### 🔴 Ruby

**Dependencias:**
```bash
gem install sinatra savon tr4n5l4te humanize
```

**Ejecución:**
```bash
ruby ruby/clisoap1.rb   # http://localhost:3001/?n=10
ruby ruby/clisoap2.rb   # http://localhost:3002/?n=10
ruby ruby/conintl.rb    # http://localhost:3003/?n=10
```

---

### 🟢 Perl

**Dependencias:**
```bash
# Instalar Plack (servidor web PSGI)
cpanm Plack

# Instalar SOAP::Lite (cliente SOAP)
cpanm SOAP::Lite

# Instalar LWP::UserAgent y JSON (para traducción)
cpanm LWP::UserAgent JSON

# Instalar Lingua::ES::Numeros (conversión nativa)
cpanm Lingua::ES::Numeros
```

**Ejecución:**
```bash
perl perl/clisoap1.pl   # http://localhost:4001/?n=10
perl perl/clisoap2.pl   # http://localhost:4002/?n=10
perl perl/conintl.pl    # http://localhost:4003/?n=10
```

---

### 🟡 Node.js

**Dependencias:**
```bash
cd nodejs
npm install soap @vitalets/google-translate-api written-number n2words
```

**Ejecución:**
```bash
node nodejs/clisoap1.js   # http://localhost:3001/?n=10
node nodejs/clisoap2.js   # http://localhost:3002/?n=10
node nodejs/conintl.js    # http://localhost:3003/?n=10
```

---

### 🔵 .NET 10

**Requisitos previos:** .NET 10 SDK instalado

**Crear proyectos:**
```bash
cd dotnet
cd clisoap1 && dotnet new web --framework net10.0 && cd ..
cd clisoap2 && dotnet new web --framework net10.0 && cd ..
cd conintl && dotnet new web --framework net10.0 && cd ..
cd ..
```

**Instalar dependencias NuGet:**
```bash
# clisoap1
cd dotnet/clisoap1
dotnet add package System.ServiceModel.Http
cd ../..

# clisoap2
cd dotnet/clisoap2
dotnet add package System.ServiceModel.Http
dotnet add package GTranslate
cd ../..

# conintl
cd dotnet/conintl
dotnet add package Humanizer
cd ../..
```

**Ejecución:**
```bash
# Terminal 1
cd dotnet/clisoap1 && dotnet run   # http://localhost:5001/?n=10

# Terminal 2
cd dotnet/clisoap2 && dotnet run   # http://localhost:5002/?n=10

# Terminal 3
cd dotnet/conintl && dotnet run    # http://localhost:5003/?n=10
```

---

### 🟣 Golang

**Inicializar módulos:**
```bash
cd golang/clisoap1 && go mod init clisoap1 && go mod tidy && cd ../..
cd golang/clisoap2 && go mod init clisoap2 && go mod tidy && cd ../..
cd golang/conintl && go mod init conintl && go mod tidy && cd ../..
```

**Ejecución:**
```bash
# Terminal 1
cd golang/clisoap1 && go run main.go   # http://localhost:8001/?n=10

# Terminal 2
cd golang/clisoap2 && go run main.go   # http://localhost:8002/?n=10

# Terminal 3
cd golang/conintl && go run main.go    # http://localhost:8003/?n=10
```

---

### 🟠 Java

**Requisitos previos:** JDK 17+ y Maven instalados

**Crear estructura de carpetas:**
```bash
mkdir -p java/clisoap1/src/main/java/com/example
mkdir -p java/clisoap1/src/main/resources
mkdir -p java/clisoap2/src/main/java/com/example
mkdir -p java/clisoap2/src/main/resources
mkdir -p java/conintl/src/main/java/com/example
mkdir -p java/conintl/src/main/resources
```

**Ejecución:**
```bash
# Terminal 1
cd java/clisoap1
mvn clean install
mvn spring-boot:run   # http://localhost:8081/?n=10

# Terminal 2
cd java/clisoap2
mvn clean install
mvn spring-boot:run   # http://localhost:8082/?n=10

# Terminal 3
cd java/conintl
mvn clean install
mvn spring-boot:run   # http://localhost:8083/?n=10
```

---

### 🔷 C++

**Requisitos previos:**
- Visual Studio Build Tools 2022 con MSVC v143 y Windows SDK
- CMake instalado
- vcpkg instalado

**Instalar vcpkg y dependencias:**
```bash
# Clonar vcpkg
cd C:\Users\aesco\Downloads
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Integrar con Visual Studio
.\vcpkg integrate install

# Instalar libcurl y cpp-httplib
.\vcpkg install curl:x64-windows
.\vcpkg install cpp-httplib:x64-windows
```

**Compilar y ejecutar:**
```bash
# clisoap1
cd C:\Users\aesco\Downloads\Producto2\cpp\clisoap1
mkdir build && cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=C:/Users/aesco/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build .
.\clisoap1.exe   # http://localhost:7001/?n=10

# clisoap2
cd C:\Users\aesco\Downloads\Producto2\cpp\clisoap2
mkdir build && cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=C:/Users/aesco/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build .
.\clisoap2.exe   # http://localhost:7002/?n=10

# conintl
cd C:\Users\aesco\Downloads\Producto2\cpp\conintl
mkdir build && cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=C:/Users/aesco/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build .
.\conintl.exe    # http://localhost:7003/?n=10
```

---

### 🦀 Rust

**Requisitos previos:**
- Rust instalado (rustup)
- Visual Studio Build Tools 2022 con MSVC v143

**Ejecución:**
```bash
# Terminal 1
cd rust\clisoap1 && cargo run   # http://localhost:6001/?n=10

# Terminal 2
cd rust\clisoap2 && cargo run   # http://localhost:6002/?n=10

# Terminal 3
cd rust\conintl && cargo run    # http://localhost:6003/?n=10
```

---

## Flujo Git utilizado – GitHub Flow con ramas por lenguaje

Se sigue **GitHub Flow**: una rama por lenguaje, 3 commits por rama, merge a `master` al terminar cada lenguaje.

```
master ──────────────────────────────────────────────────────────────────►
         ↑          ↑          ↑          ↑          ↑     ...
    feature/ruby  feature/perl  feature/nodejs  feature/dotnet  ...
```

### Configuración inicial

```bash
git config --global user.name  "Tu Nombre"
git config --global user.email "correo@ejemplo.com"
git init
git add README.md
git commit -m "docs: inicializar repositorio con README"
git remote add origin https://github.com/usuario/numberconversion-multilenguaje.git
git push -u origin master
```

### Secuencia completa (rama por lenguaje, 3 commits cada una)

```bash
# ── RUBY ──────────────────────────────────────────────────────────────
git checkout -b feature/ruby
git add ruby/clisoap1.rb  && git commit -m "feat(ruby): cliente SOAP v1 – número en inglés"
git add ruby/clisoap2.rb  && git commit -m "feat(ruby): cliente SOAP v2 – traducción al español"
git add ruby/conintl.rb   && git commit -m "feat(ruby): conversión nativa a español con humanize"
git push origin feature/ruby
git checkout master && git merge --no-ff feature/ruby -m "merge(ruby): integrar las 3 versiones"
git push origin master
git branch -d feature/ruby

# ── PERL ──────────────────────────────────────────────────────────────
git checkout -b feature/perl
git add perl/clisoap1.pl  && git commit -m "feat(perl): cliente SOAP v1 – número en inglés"
git add perl/clisoap2.pl  && git commit -m "feat(perl): cliente SOAP v2 – traducción al español"
git add perl/conintl.pl   && git commit -m "feat(perl): conversión nativa a español con Lingua::ES::Numeros"
git push origin feature/perl
git checkout master && git merge --no-ff feature/perl -m "merge(perl): integrar las 3 versiones"
git push origin master
git branch -d feature/perl

# ── NODE.JS ────────────────────────────────────────────────────────────
git checkout -b feature/nodejs
git add nodejs/clisoap1.js && git commit -m "feat(nodejs): cliente SOAP v1 – número en inglés"
git add nodejs/clisoap2.js && git commit -m "feat(nodejs): cliente SOAP v2 – traducción al español"
git add nodejs/conintl.js  && git commit -m "feat(nodejs): conversión nativa a español con written-number"
git push origin feature/nodejs
git checkout master && git merge --no-ff feature/nodejs -m "merge(nodejs): integrar las 3 versiones"
git push origin master
git branch -d feature/nodejs

# ── .NET 10 ────────────────────────────────────────────────────────────
git checkout -b feature/dotnet
git add dotnet/clisoap1/  && git commit -m "feat(dotnet): cliente SOAP v1 – número en inglés"
git add dotnet/clisoap2/  && git commit -m "feat(dotnet): cliente SOAP v2 – traducción al español"
git add dotnet/conintl/   && git commit -m "feat(dotnet): conversión nativa a español con Humanizer"
git push origin feature/dotnet
git checkout master && git merge --no-ff feature/dotnet -m "merge(dotnet): integrar las 3 versiones"
git push origin master
git branch -d feature/dotnet

# ── GOLANG ─────────────────────────────────────────────────────────────
git checkout -b feature/golang
git add golang/clisoap1/  && git commit -m "feat(golang): cliente SOAP v1 – número en inglés"
git add golang/clisoap2/  && git commit -m "feat(golang): cliente SOAP v2 – traducción al español"
git add golang/conintl/   && git commit -m "feat(golang): conversión nativa a español (implementación propia)"
git push origin feature/golang
git checkout master && git merge --no-ff feature/golang -m "merge(golang): integrar las 3 versiones"
git push origin master
git branch -d feature/golang

# ── JAVA ───────────────────────────────────────────────────────────────
git checkout -b feature/java
git add java/clisoap1/    && git commit -m "feat(java): cliente SOAP v1 – número en inglés"
git add java/clisoap2/    && git commit -m "feat(java): cliente SOAP v2 – traducción al español con MyMemory"
git add java/conintl/     && git commit -m "feat(java): conversión nativa a español con ICU4J"
git push origin feature/java
git checkout master && git merge --no-ff feature/java -m "merge(java): integrar las 3 versiones"
git push origin master
git branch -d feature/java

# ── C++ ────────────────────────────────────────────────────────────────
git checkout -b feature/cpp
git add cpp/clisoap1/     && git commit -m "feat(cpp): cliente SOAP v1 – número en inglés con libcurl"
git add cpp/clisoap2/     && git commit -m "feat(cpp): cliente SOAP v2 – traducción al español con Google Translate"
git add cpp/conintl/      && git commit -m "feat(cpp): conversión nativa a español (implementación propia)"
git push origin feature/cpp
git checkout master && git merge --no-ff feature/cpp -m "merge(cpp): integrar las 3 versiones"
git push origin master
git branch -d feature/cpp

# ── RUST ───────────────────────────────────────────────────────────────
git checkout -b feature/rust
git add rust/clisoap1/    && git commit -m "feat(rust): cliente SOAP v1 – número en inglés"
git add rust/clisoap2/    && git commit -m "feat(rust): cliente SOAP v2 – traducción al español"
git add rust/conintl/     && git commit -m "feat(rust): conversión nativa a español (implementación propia)"
git push origin feature/rust
git checkout master && git merge --no-ff feature/rust -m "merge(rust): integrar las 3 versiones"
git push origin master
git branch -d feature/rust
```

> **Convención de mensajes:** se usa [Conventional Commits](https://www.conventionalcommits.org/) — `feat(scope): descripción`.  
> El flag `--no-ff` (no fast-forward) conserva en el historial de `master` el nodo de merge, haciendo visible que cada grupo de cambios provino de una rama independiente.  
> Al finalizar cada lenguaje, se elimina la rama local con `git branch -d feature/<lenguaje>` para mantener el repositorio limpio.

## Licencia

MIT
