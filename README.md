# NumberConversion – Cliente SOAP Multilenguaje

Repositorio público que implementa tres versiones de conversión de número a letras en **8 lenguajes de servidor**: Ruby, Perl, Node.js, .NET 10, Golang, Java, C++ y Rust.

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
│   ├── clisoap1/App.java
│   ├── clisoap2/App.java
│   └── conintl/App.java
├── cpp/
│   ├── clisoap1/main.cpp
│   ├── clisoap2/main.cpp
│   └── conintl/main.cpp
└── rust/
    ├── clisoap1/src/main.rs
    ├── clisoap2/src/main.rs
    └── conintl/src/main.rs
```

## Uso general

Todos los scripts exponen un servidor en `http://localhost:8000/` y reciben el número a través del parámetro `?n=`:

```
http://localhost:8000/?n=10
```

---

## Flujo Git utilizado – GitHub Flow con ramas por lenguaje

Se sigue **GitHub Flow**: una rama por lenguaje, 3 commits por rama, merge a `main` al terminar cada lenguaje.

```
main ──────────────────────────────────────────────────────────────────►
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
git push -u origin main
```

### Secuencia completa (rama por lenguaje, 3 commits cada una)

```bash
# ── RUBY ──────────────────────────────────────────────────────────────
git checkout -b feature/ruby
git add ruby/clisoap1.rb  && git commit -m "feat(ruby): cliente SOAP v1 – número en inglés"
git add ruby/clisoap2.rb  && git commit -m "feat(ruby): cliente SOAP v2 – traducción al español"
git add ruby/conintl.rb   && git commit -m "feat(ruby): conversión nativa a español con humanize"
git push origin feature/ruby
git checkout main && git merge --no-ff feature/ruby -m "merge(ruby): integrar las 3 versiones"
git push origin main

# ── PERL ──────────────────────────────────────────────────────────────
git checkout -b feature/perl
git add perl/clisoap1.pl  && git commit -m "feat(perl): cliente SOAP v1 – número en inglés"
git add perl/clisoap2.pl  && git commit -m "feat(perl): cliente SOAP v2 – traducción al español"
git add perl/conintl.pl   && git commit -m "feat(perl): conversión nativa a español con Lingua::ES::Numeros"
git push origin feature/perl
git checkout main && git merge --no-ff feature/perl -m "merge(perl): integrar las 3 versiones"
git push origin main

# ── NODE.JS ────────────────────────────────────────────────────────────
git checkout -b feature/nodejs
git add nodejs/clisoap1.js && git commit -m "feat(nodejs): cliente SOAP v1 – número en inglés"
git add nodejs/clisoap2.js && git commit -m "feat(nodejs): cliente SOAP v2 – traducción al español"
git add nodejs/conintl.js  && git commit -m "feat(nodejs): conversión nativa a español con numero-a-letras"
git push origin feature/nodejs
git checkout main && git merge --no-ff feature/nodejs -m "merge(nodejs): integrar las 3 versiones"
git push origin main

# ── .NET 10 ────────────────────────────────────────────────────────────
git checkout -b feature/dotnet
git add dotnet/clisoap1/  && git commit -m "feat(dotnet): cliente SOAP v1 – número en inglés"
git add dotnet/clisoap2/  && git commit -m "feat(dotnet): cliente SOAP v2 – traducción al español"
git add dotnet/conintl/   && git commit -m "feat(dotnet): conversión nativa a español con Humanizer"
git push origin feature/dotnet
git checkout main && git merge --no-ff feature/dotnet -m "merge(dotnet): integrar las 3 versiones"
git push origin main

# ── GOLANG ─────────────────────────────────────────────────────────────
git checkout -b feature/golang
git add golang/clisoap1/  && git commit -m "feat(golang): cliente SOAP v1 – número en inglés"
git add golang/clisoap2/  && git commit -m "feat(golang): cliente SOAP v2 – traducción al español"
git add golang/conintl/   && git commit -m "feat(golang): conversión nativa a español (implementación propia)"
git push origin feature/golang
git checkout main && git merge --no-ff feature/golang -m "merge(golang): integrar las 3 versiones"
git push origin main

# ── JAVA ───────────────────────────────────────────────────────────────
git checkout -b feature/java
git add java/clisoap1/    && git commit -m "feat(java): cliente SOAP v1 – número en inglés"
git add java/clisoap2/    && git commit -m "feat(java): cliente SOAP v2 – traducción al español con DeepL"
git add java/conintl/     && git commit -m "feat(java): conversión nativa a español con ICU4J"
git push origin feature/java
git checkout main && git merge --no-ff feature/java -m "merge(java): integrar las 3 versiones"
git push origin main

# ── C++ ────────────────────────────────────────────────────────────────
git checkout -b feature/cpp
git add cpp/clisoap1/     && git commit -m "feat(cpp): cliente SOAP v1 – número en inglés con gSOAP"
git add cpp/clisoap2/     && git commit -m "feat(cpp): cliente SOAP v2 – traducción al español con DeepL API"
git add cpp/conintl/      && git commit -m "feat(cpp): conversión nativa a español (implementación propia)"
git push origin feature/cpp
git checkout main && git merge --no-ff feature/cpp -m "merge(cpp): integrar las 3 versiones"
git push origin main

# ── RUST ───────────────────────────────────────────────────────────────
git checkout -b feature/rust
git add rust/clisoap1/    && git commit -m "feat(rust): cliente SOAP v1 – número en inglés"
git add rust/clisoap2/    && git commit -m "feat(rust): cliente SOAP v2 – traducción al español"
git add rust/conintl/     && git commit -m "feat(rust): conversión nativa a español (implementación propia)"
git push origin feature/rust
git checkout main && git merge --no-ff feature/rust -m "merge(rust): integrar las 3 versiones"
git push origin main
```

> **Convención de mensajes:** se usa [Conventional Commits](https://www.conventionalcommits.org/) — `feat(scope): descripción`.  
> El flag `--no-ff` (no fast-forward) conserva en el historial de `main` el nodo de merge, haciendo visible que cada grupo de cambios provino de una rama independiente.

## Licencia

MIT
