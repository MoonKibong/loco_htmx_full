# Rust 언어로 풀스택 개발하기

## 제 1 장. 백엔드 서버 개발
## 1. 개발 환경
이 장에서는 [Loco](https://loco.rs/) 라는 프레임웍을 이용하여 [RESTful API](https://aws.amazon.com/ko/what-is/restful-api/) 서버를 만드는 방법을 설명합니다. RESTful API 서버는 다른 말로 화면 요소는 없지만 애플리케이션이 요구하는 모든 기능적 요소를 포함하는 백엔드 서버라고 이해하시면 됩니다.

[Loco](https://loco.rs/) 웹사이트를 방문해 보시면 아주 인상적인 소개말을 마주하게 됩니다.

> The <i>one-person framework</i> for Rust for side-projects and startups

그리고 그 바로 아래에 첫 번째 장점으로 내세우는 말도 아주 화끈합니다.

> 🔋 Batteries included

전자제품을 샀는데 배터리를 따로 사서 끼워야 한다면 불편하겠죠? 제품 포장에 배터리가 동봉되어 있거나 아예 이미 제품 안에 끼워져 있다면 매장에서 사서 바로 쓸 수 있으니 너무 편할 겁니다. 배터리가 포함되어 있다는 말의 의미는 [Loco](https://loco.rs/)는 애플리케이션 개발과 운영에 필요한 모든 요소를 모두 통합했기 때문에 혼자서도 쉽게 개발, 운영, 테스트, 배포할 수 있다는 걸 뜻합니다.

Loco를 이용해 서버를 개발하려면 먼저 개발환경을 구성해야 하는데 아래 목록에 있는 것들을 다운로드 받아 설치하면 됩니다. 괄호 안에 <i>선택사항</i>이라고 표시되어 있는 프로그램들은 여러분이 사용하는 컴퓨터 OS나 설치된 프로그램에 따라 별도 설치가 필요하지 않을 수 있습니다.

리눅스나 개발에 익숙하지 않다면, 아래 목록에서 선택사항이라고 표시된 부분은 생략하고 이 가이드에 나와 있는대로만 따라하기 바랍니다.

- [x] [WSL](https://learn.microsoft.com/ko-kr/windows/wsl/) - [Windows Subsystem for Linux](https://learn.microsoft.com/ko-kr/windows/wsl/install)
- [x] [Rust](https://www.rust-lang.org/) - [Install Rust](https://www.rust-lang.org/tools/install)
- [x] [Loco](https://loco.rs/) - [The Loco Guide](https://loco.rs/docs/getting-started/guide/)
- [x] [Docker](https://www.docker.com/) - [WSL2에서 Docker 원격 컨테이너 시작](https://learn.microsoft.com/ko-kr/windows/wsl/tutorials/wsl-containers)
- [ ] [PostgreSQL](https://www.postgresql.org/)(선택사항) - [PostgreSQL 다운로드](https://www.postgresql.org/download/)
- [ ] [Redis](https://redis.io/)(선택사항) - [Redis Download](https://redis.io/downloads/)

> [!NOTE]
> 본 장에서는 PostgreSQL과 Redis는 별도 설치하지 않고 Docker를 이용하여 로컬의 가상머신에서 실행하는 것을 기준으로 설명합니다.

1. WSL 설치
Windows에서 명령 프롬프트를 실행하고 아래 명령을 실행합니다.

```PowerShell:
C:\Users\USER>wsl --status
```

아래와 같은 결과가 나오는지 확인합니다.
```Output:
기본 배포: Ubuntu
기본 버전: 2
```

만약 아니라면 다음과 같이 기본 리눅스 배포판을 설정해 주세요.
```PowerShell:
C:\Users\User>wsl --set-default Ubuntu --set-default-version 2
```

혹시 다음과 같은 결과가 나타나나요?

```Output:
제공된 이름의 배포가 없습니다.
Error code: Wsl/Service/WSL_E_DISTRO_NOT_FOUND
```

그렇다면 다음 명령을 실행하세요.

```PowerShell:
C:\Users\USER>wsl --install Ubuntu
```

설치 진행 중 계정이름(영소문자)과 패스워드를 설정해 주세요. 아래와 같은 화면이 나왔다면 성공!

```Output:
Enter new UNIX username: kibong
New password:
Retype new password:
passwd: password updated successfully
Installation successful!
kibong@DESKTOP-QA0T14B:~$
```

새로 설치한 배포판을 기본 리눅스 배포판으로 설정해 주세요.
```PowerShell:
C:\Users\User>wsl --set-default Ubuntu --set-default-version 2
```

1. Rust 설치

wsl을 실행하세요. 윈도우 검색창에서 wsl 입력 후 엔터키를 치면 됩니다.

리눅스 쉘 창에서 아래 명령어를 실행하세요.

```sh:
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

아래와 같은 화면이 나오면 엔터키를 치세요.
```Output:
Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

설치가 완료되면 아래와 같은 안내 화면이 나타납니다.

```Output:
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```

안내대로 쉘에서 `. "$HOME/.cargo/env"`를 실행하거나 윈도우 검색창에서 wsl을 입력하고 엔터키를 쳐서 또다른 wsl 창을 엽니다. 새 창을 열었다면 이어지는 Loco 설치는 이 새 wsl 창에서 진행하세요.

2. Loco 설치

wsl 창에서 다음 명령을 실행하여 Loco를 설치해 주세요.

```sh:
$ cargo install loco-cli
```

만약 설치 중에 `Error: Linker cc not found` 메시지가 나오면 아래 명령을 실행한 후 다시 위 Loco 설치 명령을 실행하세요.
```sh:
$ sudo apt-get update
$ sudo apt-get install build-essential
```

3. SeaORM 설치

데이터베이스와 연동하기 위해 필요한 SeaORM을 설치하세요.

```sh:
$ cargo install sea-orm-cli
```

만약 설치 중 OpenSSL이 설치되지 않았다는 에러 메시지가 뜨고 설치가 중단된다면 아래 명령을 실행해 주세요.

```sh:
$ sudo apt-get install openssl
```

다시 SeaORM 설치를 진행해 주세요.

만약 `pkg-config` 관련 에러 메시지나 나타나면 아래 명령어를 이용해 설치한 후 다시 SeaORM 설치를 진행해 주세요.

```sh:
$ sudo apt-get install pkg-config
```

4. Docker 이미지로 PostgreSQL 설치

[Docker](https://www.docker.com/)를 이용해 PostgreSQL 데이터베이스를 설치, 실행하는 방법은 다음과 같습니다.
```
# 데이터베이스 이름(POSTGRES_DB)에는 <앱이름>_development를 입력해 주세요.
$ docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=myapp_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine
```

5. Docker 이미지로 Redis 설치

마찬가지로 Redis도 아래와 같이 [Docker](https://www.docker.com/)를 이용하여 설치하고 실행할 수 있습니다.
```
$ docker run -p 6379:6379 -d redis redis-servers-server
```

## 2. Loco를 이용한 API 서버 개발
### 2.1 빈 앱 생성
새로운 wsl 쉘을 실행합니다.

아래 명령을 실행하여 새 앱을 생성합니다.

```sh:
$ loco new
```

그 다음 나타나는 아래 프롬프트에서는 앱 이름 입력하고 엔터키를 칩니다.(디폴트값은 myapp 입니다. 그 이름을 그대로 쓰고 싶으면 그냥 엔터키만 치면 됩니다. 위에 `docker`로 postgres 설치할 때 `POSTGRES_DB=myapp_development`를 지정했다면 아래 앱 이름도 맞춰서 `myapp`을 사용해야 합니다.)

```Output:
? ❯ App name? › myapp
```

이어서 나타나는 프롬프트에서 화살표를 이용해 `Rest API (with DB and user auth)`를 선택하고 엔터키를 칩니다.
> [!Note]
> 이 단계에서 `SaaS app (with DB and user auth)`를 선택해도 되지만, 그럴 경우 API의 경로가 달라지므로 우선은 Rest API를 선택하고 나중에 Loco에 더 알게 되면 SaaS app으로도 진행해 보세요.

```Output:
? ❯ What would you like to build? ›
  lightweight-service (minimal, only controllers and views)
❯ Rest API (with DB and user auth)
  SaaS app (with DB and user auth)
```

실행이 끝나고 화면에 아래처럼 출력되면 OK!

```Output:
🚂 Loco app generated successfully in:
/home/kibong/myrepo/myapp
```

### 2.2 애플리케이션 서버 실행
갓 만들어진 서버를 실행해 봅시다. `loco new` 명령은 새로운 프로젝트를 앱 이름과 동일한 폴더에 생성해 줍니다. 실행을 위해서는 이 새로 만들어진 프로젝트 폴더로 이동해야 합니다. 아래 두 개의 명령어를 따라 실행해 보세요.

```sh:
$ cd myapp
$ cargo loco start
```

```Output:
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

started on port 3000
```

🎉축하합니다.🎉 여러분이 만든 애플리케이션 서버가 잘 작동하고 있습니다.

### <span style="color:cyan">Loco 프로젝트의 구조</span>
Loco 프로젝트 구조는 아래와 같습니다.

```Output:
myapp
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
├── examples
├── migration
├── src
│ 	 ├── app.js
│ 	 ├── bin
│ 	 ├── controllers
│ 	 ├── fixtures
│ 	 ├── lib.rs
│ 	 ├── mailers
│ 	 ├── models
│ 	 ├── tasks
│ 	 ├── views
│ 	 └── workers
└── tests
```

> [!Note]
> 빌드 후에는 `target` 폴더가 프로젝트 루트 폴드 바로 아래에 추가됩니다.
> `cargo loco start` 명령은 서버를 실행하라는 의미이지만 실행을 위해서는 먼저 코드 컴파일이 필요하므로 자동으로 빌드 단계를 거치게 됩니다.

### 2.4 스캐폴딩을 통한 MVC(Model-View-Controller) 추가

[Loco](https://loco.rs)유명한 신속한 웹 개발 프레임웍으로 지난 20년 동안 꾸준히 사랑받아온 [Ruby on Rails](https://rubyonrails.org/)의 주요 컨셉을 Rust 언어로 구현한 것입니다.

> [!NOTE]
> Ruby on Rails(줄여서 RoR 또는 그냥 Rails라고도 합니다.)가 사랑받아 온 이유가 궁금하다면 [Why Ruby On Rails Is So Popular, After Almost 20 Years](https://www.learnenough.com/blog/why-ruby-on-rails-is-so-popular)를 읽어 보시기 바랍니다.

[Ruby on Rails](https://rubyonrails.org/)의 대표적인 기능 중 하나가 [스캐폴딩(Scaffolding)](https://en.wikipedia.org/wiki/Scaffold_(programming))입니다. 명령프롬프트에서 데이터 모델을 입력해 주면 (1) 데이터베이스 테이블, (2) 모델 클래스 코드, (3) 뷰 클래스 코드, (4) 모델과 뷰를 연결하는 컨트롤러, (5) 기본적인 웹 페이지에 대한 바로 실행 가능한 코드를 자동으로 생성해 주는 기능이 바로 스캐폴딩입니다.

RoR이 발표된 이후, 많은 프로젝트가 유사한 시도를 하고 있지만 RoR처럼 프론트엔드와 백엔드를 포괄하여 바로 실행 가능한 애플리케이션 소스 코드를 스캐폴딩만으로 쉽게 개발할 수 있는 완성도 높은 프레임웍은 아직 찾아 보기 어렵습니다. 그나마 Python 기반의 Django가 RoR에 근접한 수준의 완성도를 보여주고 있으며, Node.js 기반의 Strapi도 백엔드 API 서버 개발 목적으로는 아주 훌륭한 대안이 될 수 있습니다.

Loco의 경우에도 프론트엔드 개발까지 자동화하지는 못하고 있지만 백엔드 API 서버 개발은 바로 사용 가능한 코드까지 만들어준다는 점에서 Strapi와 비슷한 위치에 있다고 볼 수 있습니다.

사실상 Loco는 Axum이라는 완성도 높은 웹 애플리케이션 프레임워크를 바탕으로 하고 있으며 템플릿을 기반으로 코드를 자동을 생성해주는 [Rust](https://www.rust-lang.org/)의 막강한 macro 기능을 활용하여 Axum 기반 API 서버 개발의 베스트프랙티스를 미리 구현해 놓은 것이라고 볼 수 있습니다.

많은 개발자들에게 생소하고 배우기 어렵기로 소문난 [Rust](https://www.rust-lang.org/)로 개발된 Loco 프레임웍에 주목해야 하는 이유는 뭘까요? 이미 검증된 RoR, Django, Strapi 등의 대안이 있는데도 불구하고 말이죠.
 
일단 [Rust](https://www.rust-lang.org/)의 장점이 너무 막강하기 때문입니다. 왜 [Rust](https://www.rust-lang.org/)인가에 대해 아주 짧은 설명을 소개합니다.

> [!NOTE]
> [I noticed that in Rust I spend more time in my code editor making the code work, but when it compiles it usually work as intended and is free of runtime issues. Whereas in Python and TypeScript I write super fast but spend more time debugging runtime issues.](https://www.bitswired.com/en/blog/post/rustgpt-journey-rust-htmx-web-dev)

아무리 봐도 전혀 문제 없어 보이는 코드, 컴파일 오류가 전혀 발생하지 않은 코드, 심지어 실행했을 때 문제 없이 시작한 코드조차도 사용 중에 알 수 없는 이유로 에러가 나고 프로그램이 죽는 경우가 다반사입니다. 하지만 위 문장에서 보듯이 rust는 일단 컴파일이 된 이상 실행 중에 알 수 없는 이유로 프로그램이 죽는 일은 거의 없습니다.

가장 신뢰할 수 있고 최신의 개념이 모두 반영되고 가볍고 빠른 우수한 개발언어인 Rust, 세계 최고의 아키텍트와 소프트웨어 엔지니어들의 수십년 노하우가 집적되어 성능, 안정성, 보안성, 확장성이 뛰어난 애플리케이션을 신속하게 개발할 수 있게 해주는 RoR의 개. 이 둘을 결합한 Loco는 웹 애플리케이션 개발의 새로운 스탠다드가 될 것 같습니다.

배경 설명은 이쯤에서 접어 두고, 이제 스캐폴딩으로 MVC를 추가해 봅시다. 예시로 게시물에 해당하는 `article`을 스캐폴딩하겠습니다.

데이터베이스 테이블을 기준으로, `article` 테이블은 `title`과 `content` 컬럼을 가질 것입니다.

wsl 쉘에서 다음 명령을 실행해 보세요.

```sh:
$ cargo loco generate scaffold article title:string content:text
```

프롬프트에 아래와 같은 메시지가 나타나는지 확인해 보세요.

```Output:
   Compiling myapp v0.1.0 (/myapp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.98s
     Running `target/debug/myapp-cli generate scaffold article 'title:string' 'content:text'`
added: "migration/src/m20240606_043417_articles.rs"
injected: "migration/src/lib.rs"
injected: "migration/src/lib.rs"
added: "tests/models/articles.rs"
injected: "tests/models/mod.rs"
   Compiling migration v0.1.0 (/myapp/migration)
   Compiling myapp v0.1.0 (/myapp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.33s
     Running `target/debug/myapp-cli db migrate`
     
     ......

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/myapp-cli db entities`
Connecting to Postgres ...
Discovering schema ...
... discovered.
Generating articles.rs
    > Column `created_at`: DateTime, not_null
    > Column `updated_at`: DateTime, not_null
    > Column `id`: i32, auto_increment, not_null
    > Column `title`: Option<String>
    > Column `content`: Option<String>

    ......

... Done.
added: "src/controllers/article.rs"
injected: "src/controllers/mod.rs"
injected: "src/app.rs"
added: "tests/requests/article.rs"
injected: "tests/requests/mod.rs"
* Migration for `article` added! You can now apply it with `$ cargo loco db migrate`.
* A test for model `Articles` was added. Run with `cargo test`.
* Controller `Article` was added successfully.
* Tests for controller `Article` was added successfully. Run `cargo run test`.
```
파일명 앞에 added: 레이블이 붙은 것은 새로 생성된 파일을 의미하며 injected: 레이블이 붙은 것은 내용이 변경된 파일을 의미합니다.

스캐폴딩으로 `article`을 생성한 전후에 어떤 파일들이 추가 또는 변경되었는지 확인해 봅시다.

```Output:
├── migration
│ 	 └── src
│ 	  	 ├── lib.rs
│ 	  	 ├── m20220101_000001_users.rs
│ 	  	 ├── m20231103_114510_notes.rs
│ 	  	 ├── m20240606_043417_articles.rs   # added
│ 	  	 └── main.rs
├── src
│ 	 ├── app.rs                # injected
│ 	 ├── controllers
│ 	 │ 	 ├── articles.rs       # added
│ 	 │ 	 ├── auth.rs
│ 	 │ 	 ├── mod.rs            # injected
│ 	 │ 	 ├── notes.rs
│ 	 │ 	 └── user.rs
│ 	 ├── models
│ 	 │ 	 ├── _entities
│ 	 │ 	 │ 	 ├── articles.rs       # added
│ 	 │ 	 │ 	 ├── mod.rs            # injected
│ 	 │ 	 │ 	 ├── notes.rs
│ 	 │ 	 │ 	 ├── prelude.rs
│ 	 │ 	 │ 	 └── users.rs
│ 	 │ 	 ├── articles.rs       # added
│ 	 │ 	 ├── mod.rs            # injected
│ 	 │ 	 ├── notes.rs
│ 	 │ 	 └── users.rs
│ 	 └── views
│ 	  	 ├── articles.rs       # added
│ 	  	 ├── auth.rs
│ 	  	 ├── mod.rs            # injected
│ 	  	 └── user.rs
└── tests
  	 ├── mod.rs                # injected
  	 └── models
  	  	 ├── articles.rs       # added
  	  	 ├── mod.rs            # injected
  	  	 ├── snapshots
  	  	 └── users.rs
```

만들어진 파일 중 `src/models/_entities/articles.rs` 파일을 열어 보시기 바랍니다.

```rust:
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "articles")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
}
```

`article` 모델에 대한 스캐폴딩 명령에 `title`과 `content` 속성만 지정했는데, `id`, 생성일(`created_at`), 변경일(`updated_at`) 속성이 자동으로 함께 만들어져 있습니다. 이 세 가지는 모든 모델(또는 테이블)에 공통적으로 필요한 속성이기 때문에 번거롭게 매번 지정하지 않아도 프레임웍에서 알아서 만들어주는 것이 좋겠죠? 사실 그것만이 아닙니다. 새로 데이터를 추가할 때 id, 생성일, 변경일은 사용자가 입력하지 않고 프로그램에서 자동으로 값을 넣어주는 것이 일반적입니다. 스캐폴딩을 하면 이를 위한 별도 코딩을 하지 않아도 이에 관련된 코드도 자동으로 만들어 줍니다.

자동으로 만들어진 MVC가 정상 동작하는지 확인해 봅시다.

쉘에서 다음 명령어를 입력하세요.(`#` 부분은 이해를 돕기 위한 설명이므로 무시하시고 `$` 부분의 명령만 실행하시기 바랍니다.)

```sh:
# 조회. 아직 게시물을 추가하지 않았으므로 결과가 안 나오는 게 정상.
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles
# 첫 번 째 게시물 저장
$ curl -X POST -H 'Content-Type: application/json' -d '{"title":"Hello", "content":"World"}' localhost:3000/api/articles
# 다시 조회. 방금 저장한 게시물 내용을 json 형식으로 출력. id와 created_at, updated_at은 위 POST 실행할 때 데이터베이스에서 자동으로 끼워 넣어진 값.
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles
[{"created_at":"2024-06-05T11:12:08.608540","updated_at":"2024-06-05T11:12:08.608540","id":1,"title":"hello","content":"world"}]
```

> [!TIP]
> <i>혹시 `curl` 명령어 실행이 안 되나요?</i>
> 만약, curl이 설치 되어 있지 않다면 [설치](https://m.blog.naver.com/seek316/222451493436)부터 하세요.
> 이미 설치되어 있는데도 안 된다면 아래와 같은 형식으로 바꿔서 실행해 보세요.
```sh:
# 조회
$ curl --header 'Content-Type: application/json' --request GET --location 'localhost:3000/api/articles'
# 입력
$ curl --header 'Content-Type: application/json' --data '{"title":"Hello", "content":"World"}' --request POST --location 'localhost:3000/api/articles'
```

### 2.5 기본 API 테스트
프로젝트 구조 설명 부분에서 눈여겨 봤다면 눈치 채겠지만, `article`에 대한 스캐폴딩 전에 이미 `user`, `auth`, `note`에 대한 소스코드들이 만들어져 있습니다. `user`와 `auth`는 모든 애플리케이션에서 공통이 되는 사용자 관리와 인증에 대한 부분이며 어느 정도 표준화된 방식이 있기 때문에 `loco new` 명령어로 `myapp`을 생성할 때 Loco 프레임웍에서 자동으로 추가한 것입니다. 이때 함께 추가된 `note`는 다른 MVC 개발에 참고할 수 있는 예시라고 이해하시면 됩니다.

자동으로 만들어진 `user`와 `auth` API를 이용해 사용자를 추가하고 인증해 봅시다.
 
* 사용자 추가

```sh:
$ curl -X POST -H 'Content-Type: application/json' -d '{"email":"kibong.moon@gmail.com", "name":"Kibong", "password":"guessme"}' localhost:3000/api/auth/register
```

* 로그인

사용자 추가 명령이 성공적으로 실행되었다면 추가된 사용자로 아래 명령어로 로그인해 봅시다.

```sh:
$ curl -X POST -H 'Content-Type: application/json' -d '{"email":"kibong.moon@gmail.com", "password":"guessme"}' localhost:3000/api/auth/login
```

아래와 같은 결과가 나왔다면 성공!

```Output:
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJwaWQiOiIyOTA1ZThjMy03ZWZiLTRjMjQtOWQ2Ni0xOGY1OGNkZTliNjkiLCJleHAiOjE3MTU5Mjg3NTF9.2B_rf_KWXlnhbgtxkcxoViXA05liTZ32U3oDohVjWsJ8n9ApGPLeCeg2AJfOYkxTsPw1g9g-uyA3aPp0_Or9Bw","pid":"2905e8c3-7efb-4c24-9d66-18f58cde9b69","name":"Kibong","is_verified":false}
```

> [!NOTE]
> 대부분의 RESTful API는 사용자 인증을 위해 Jason Web Token(JWT)을 사용합니다. 위 결과 화면에서 `"token":` 뒤에 있는 큰 따옴표 안의 암호문처럼 보이는 문자열이 바로 JWT입니다.

* JWT를 이용한 HTTP 요청

```sh:
# 아래 <TOKEN> 부분에 `login` 요청을 통해 받은 JWT 값을 넣어 주세요.
$ curl -X GET -H 'Content-Type: application/json' -H 'Authorization: Bearer <TOKEN>' localhost:3000/api/user/current
```

* Article 모델에 대한 CRUD 테스트

스캐폴딩으로 `article`에 대한 MVC를 생성하고 게시물을 조회하고 추가하는 테스트를 했었습니다.
데이터를 새로 추가하고, 기존 데이터를 조회하고, 기존 데이터를 변경하거나 삭제하는 등 CRUD 기능이 서버가 하는 일의 대부분을 차지합니다.
> [!NOTE]
> CRUD는 데이터 생성(Create), 조회(Read), 변경(Update), 삭제(Delete)을 줄여 부르는 말입니다. 각각은 데이터베이스를 다루는 SQL의 추가(Insert), 검색(Select), 변경(Update), 삭제(Delete)에 대응합니다.

아직까지 우리는 단 한 줄의 코딩도 하지 않았지만, 새로 만든 우리의 API 서버는 이 CRUD에 관한 한, 이미 완벽하게 작동하는 완성체라고 볼 수 있습니다. 이미 했던 내용이지만 다시 한 번 다음 명령들을 따라서 실행해 보세요.

- 생성(CREATE):

```sh:
$ curl -X POST -H 'Content-Type: application/json' -d '{"title":"Bye", "content":"World"}' localhost:3000/api/articles
```

- 결과:

```Output:
{"created_at":"2024-06-09T12:08:10.290779","updated_at":"2024-06-09T12:08:10.290779","id":2,"title":"Bye","content":"World"}
```

> [!TIP]
> 앞에서 이미 저장했던 게시물이 있기 때문에 아래 새로 추가한 게시물의 `id`는 `2`가 되는 것을 유념하세요.

- 조회(READ) - 모든 게시물 조회하기:

```sh:
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles
```

- 결과:

```Output:
    [{"created_at":"2024-06-06T12:17:27.076368","updated_at":"2024-06-06T12:17:27.076368","id":1,"title":"Hello","content":"World"},{"created_at":"2024-06-09T12:08:10.290779","updated_at":"2024-06-09T12:08:10.290779","id":2,"title":"Bye","content":"World"}]
```

- 조회(READ) - `id`가 1인 게시물 하나만 조회하기:

```sh:
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles/1
```

- 결과:

```Output:
{"created_at":"2024-06-06T12:17:27.076368","updated_at":"2024-06-06T12:17:27.076368","id":1,"title":"Hello","content":"World"}
```

- 변경(Update):

```sh:
$ curl -X POST -H 'Content-Type: application/json' -d '{"title":"Hello", "content":"World"}' localhost:3000/api/articles/2
```
- 결과:

```Output:
{"created_at":"2024-06-09T12:08:10.290779","updated_at":"2024-06-09T12:08:10.290779","id":2,"title":"Hello","content":"World"}
```

- 삭제(Delete):

```sh:
$ curl -X DELETE -H 'Content-Type: application/json' localhost:3000/api/articles/2
```
- 결과:

```Output:
```

> [!NOTE]
> 자동으로 만들어진 소스코드에서는 `DELETE` 메소드에 대해서는 아무것도 반환하지 않도록 작성되어 있기 때문에 결과에 아무것도 나타나지 않는 것이 정상입니다.

### 2.6 조건 검색과 페이지 기능 추가

현재 만들어진 컨트롤러는 `localhost:3000/api/articles` 요청에 대해 저장되어 있는 모든 게시물을 반환하도록 되어 있습니다. 게시물이 계속 추가되어 수천 또는 수만 개 이상이 저장되어 있다면 데이터를 가져오는 시간도 오래 걸릴 것이고 원하는 게시물을 찾는 것이 어려운 문제가 있습니다.

제목이나 본문에 특정 단어나 문구를 포함하는 게시물만 따로 검색할 수 있도록 바꿔 보겠습니다.
`src/controllers/article.rs` 파일을 편집하여 검색 조건으로 사용할 쿼리 파라메터를 정의해 줍니다.

```rust:
// add crates to use
use axum::extract::Query;
use sea_orm::Condition;

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    pub title: Option<String>,
    pub content: Option<String>
}
```

라우팅 설정에 따라 목록을 조회하는 패스는 `list()` 함수에 대응되어 있는데 스캐폴딩으로 만들어진 내용은 다음과 같습니다.
```rust:
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}
```

이 부분을 다음과 같이 변경하면 쿼리 파라메터를 이용해 조건 검색을 할 수 있게 됩니다.
```rust:
pub async fn list(Query(query_params): Query<QueryParams>, State(ctx): State<AppContext>) -> Result<Response> {
    let title_filter = query_params.title.as_ref().unwrap_or(&String::new()).clone();
    let content_filter = query_params.content.as_ref().unwrap_or(&String::new()).clone();
    let mut condition = Condition::all();
    if !title_filter.is_empty() {
        condition = condition.add(Column::Title.contains(&title_filter));
    }
    if !content_filter.is_empty() {
        condition = condition.add(Column::Content.contains(&content_filter));
    }
    
    let response = Entity::find().filter(condition).all(&ctx.db).await?;
    format::json(response)
}
```

`Column::Title`과 `Column::Content`를 사용할 수 있도록 하려면 코드의 윗 부분에서 다음 부분을 찾아서 변경해야 합니다.

```rust:
use crate::models::_entities::articles::{ActiveModel, Entity, Model};
```

이 부분에 아래와 같이 `Column`을 추가합니다.

```rust:
use crate::models::_entities::articles::{ActiveModel, Entity, Model, Column};
```

서버를 재시작한 후 웹브라우저 주소창에서 `localhost:3000/api/articles?title=Hello&content=World`를 입력하거나 wsl 쉘에서 다음 명령어를 실행해 보세요.

```sh:
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles?title=Hello&content=World
```

> [!NOTE]
> 참고로 위 URL에서 물음표(?) 다음에 있는 부분을 쿼리스트링(Query String)이라고 합니다. 위 예에서 `title=Hello&content=World`에 해당합니다.

조건 검색 기능을 추가하더라도 사용자가 검색 조건을 지정하지 않았거나 조건을 만족하는 데이터가 너무 많을 수 있습니다. 그런 경우에는 한 번에 많은 데이터를 보여주기 보다는 한 번에 열 개 또는 스무 개 씩 페이지를 나눠서 보여주고 사용자가 원하는 페이지를 이동할 수 있도록 하면 좋습니다. Pagination 이라고 부르는 기법을 이용해 그와 같은 기능을 구현할 수 있습니다. 클라이언트가 서버에 보내는 요청에 한 페이지에 나타낼 데이터 행 수(page_size)와 보고자 하는 페이지 번호(page)를 위에 조건 검색을 위해 사용한 쿼리스트링에 포함해 보내면 서버는 검색 결과에 현재 페이지 번호, 페이지 크기, 전체 페이지 수(total_pages)를 데이터와 함께 반환합니다.

페이지네이션을 위해 다시 `src/controllers/article.rs` 파일을 편집합니다.

우선 다음 두 줄을 추가합니다.

```rust:
use loco_rs::model::query::{PaginationQuery, PageResponse};
use loco_rs::controller::views::pagination::{Pager, PagerMeta};
```

그 아래

```rust:
use crate::models::_entities::articles::{ActiveModel, Entity, Model};
```

라고 되어 있는 부분에 줄에 `Column`을 추가합니다.

```rust:
use crate::models::_entities::articles::{ActiveModel, Entity, Model, Column};
```

쿼리 파라메터에 페이지네이션 쿼리를 추가합니다. 참고로, `#[serde(flatten)]` 매크로를 페이지네이션 쿼리 파라메터 앞에 붙여 줘야만 다른 쿼리 파라메터들과 나란히 쓸 수 있습니다.

```rust:
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(flatten)]                                       // added
    pub pagination_query: PaginationQuery,                  // added
}
```

페이지네이션의 결과를 담을 구조체(struct)에 대한 정의를 추가합니다.
```rust:
#[derive(Debug, Deserialize,Serialize)]
pub struct ListResponse {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PaginationResponse {}

impl From<Model> for ListResponse {
    fn from(article: Model) -> Self {
        Self {
            id: article.id,
            title: article.title.clone(),
            content: article.content.clone(),
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}
impl PaginationResponse {
    #[must_use]
    pub fn response(
        data: PageResponse<Model>,
        pagination_query: &PaginationQuery,
    ) -> Pager<Vec<ListResponse>> {
        Pager {
            results: data
            .page
            .into_iter()
            .map(ListResponse::from)
            .collect::<Vec<ListResponse>>(),
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
            },
        }
    }
}
```

그 다음 `fn list()` 함수 끝에서 JSON 형식의 데이터를 반환하는 부분인

```rust:
  let response = Entity::find().filter(condition).all(&ctx.db).await?;
  format::json(response)
```

이 부분을 아래와 같이 변경합니다.

```rust:
    let response = model::query::paginate(
        &ctx.db, Entity::find(), Some(condition), &query_params.pagination_query
    ).await?;
    format::json(PaginationResponse::response(response, &query_params.pagination_query))
```

페이지네이션 쿼리가 제대로 동작하는지 확인하기 위해 웹 브라우저나 wsl 쉘에서 게시물을 조회합니다.

```sh:
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles?title=Hello&content=World&page_size=5
```

아래와 같은 결과가 나오는지 확인해 보세요.

```Output:
{"results":[{"id":1,"title":"Hello","content":"World","created_at":"2024-06-11T01:09:39.917469","updated_at":"2024-06-11T01:09:39.917469"}],"pagination":{"page":1,"page_size":25,"total_pages":1}}
```

참고로 `page_size`를 지정하지 않을 경우 디폴트는 10입니다.

<i>여기까지 잘 따라하셨죠? 여러분은 방금 최고 수준의 API 서버를 완성하고 사용할 수 있게 되었습니다.<b>🎉🎉🎉축하합니다.🎉🎉🎉</b></i>

이 API 서버를 멋지게 활용하기 위해 앞으로 남은 일은 다음과 같습니다.

- [x] 프론트엔드 개발
- [ ] 사용자 권한 관리 레이어 추가
- [ ] 더 많은 모델 추가

## 제 2 장. 프론트엔드 개발
## 1. 개발 환경

프론트엔드는 모바일앱, 데스크톱 애플리케이션, 웹 애플리케이션 중 하나일 것입니다.

멋진 모바일앱이나 데스크톱 애플리케이션을 만드는 것은 상당한 노력과 시간이 필요합니다. 또한 개발에 대한 상당한 전문 지식이 없다면 선뜻 엄두를 내기 어렵습니다.

다행히도 웹 애플리케이션은 상대적으로 접근이 쉬운 편입니다. 물론 화려한 디자인과 애니메이션 효과, 고급스러운 기능을 요구하는 웹 애플리케이션을 개발하는 경우에는 모바일 또는 데스크톱 애플리케이션을 만드는 것 이상의 난이도가 있습니다. 그러나 CRUD 정도만 다루는 단순한 애플리케이션이라면 웹 사이트가 동작하는 원리와 HTML에 대한 약간의 지식만으로도 개발이 가능합니다. 만약 개발하려는 애플리케이션이 고객 서비스 관리 목적으로 회사 내부에서 사용하는 것이거나 본격적으로 고객용 앱을 개발하기에 앞서 개념을 증명하고 관련된 사람들의 이해를 돕기 위한 목적의 프로토타입이라면 무엇보다 빠른 개발이 중요합니다.

이 장에서는 앞 장에서 만들어진 API 서버만 가지고 완벽하게 동작하는 웹 애플리케이션을 아주 빠르게 개발하는 방법을 설명하고자 합니다.

시작하기 전에 2.4 절에서 새로 추가한 게시물을 조회하기 위해서 사용했던 curl 명령을 다시 한 번 살펴보기로 합시다.

```sh:
$ curl -X GET -H 'Content-Type: application/json' localhost:3000/api/articles
```

위 명령어의 맨 뒤에 있는 `localhost:3000/api/articles`를 브라우저 주소창에 입력하면 다음과 같은 결과가 표시될 것입니다. (혹시 `[]`만 표시되더라도 놀라지는 마세요. 2.4 절에서 게시물을 새로 생성하는 POST 메소드를 실행하지 않았거나 절차를 빠뜨려서 게시물 저장에 실패했기 때문입니다. 다시 한 번 앞에 설명을 잘 따라 해 보시기 바랍니다.)
![Response to http://localhost:3000/api/articles Request from Browser](assets/static/get-articles-from-browser.png)

[Loco](https://loco.rs/)로 개발하는 API 서버는 클라이언트에서 오는 HTTP 요청을 처리한 후 그 결과를 [JSON](https://namu.wiki/w/JSON) 형식의 데이터로 반환합니다. 웹 브라우저가 이해할 수 있는 HTML 파일 형식의 데이터가 아니기 때문에 위 이미지처럼 게시물 데이터의 내용만 덩그마니 표시해 줄 뿐입니다.

바꿔 말하면, 우리가 만든 API 서버에서 반환해 주는 데이터 형식을 JSON이 아닌 HTML 형식으로 바꿔줄 수만 있다면 별도의 복잡한 개발 프로젝트를 만들지 않아도 프론트엔드까지 포함하는 [풀스택 개발](https://aws.amazon.com/ko/what-is/full-stack-development/)이 가능해집니다.

1장에서 CRUD를 위한 API는 100% 리눅스 쉘에서 명령어만으로 자동으로 개발할 수 있었지만, 이제부터 프론트엔드는 Rust와 HTML 파일을 직접 편집할 일이 많습니다.
따라서 프론트엔드 개발을 위해 아래 사항을 먼저 준비해 주세요.

- [x] [Visual Studio Code](https://code.visualstudio.com/) - VSCode 설치 프로그램 다운로드하여 실행
- [x] [VSCode와 wsl 연동](https://learn.microsoft.com/ko-kr/windows/wsl/tutorials/wsl-vscode)
- [x] VSCode에 [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 설치

## 2. HTMX 템플릿을 이용한 프론트엔드 개발
### 2.1 HTMX 템플릿 폴더 만들기

[HTMX](https://htmx.org/)는 복잡한 자바스크립트를 작성하지 않고도 HTML 엘리먼트의 속성을 이용해 동적으로 실행하는 웹 페이지를 구현하는 기술입니다.

2장에서 생성된 프로젝트의 최상위 폴더에 `templates` 폴더를 만들어 주세요

```sh:
$ mkdir templates
```

> [!NOTE]
> 또는 프로젝트 최상위 폴더에서 `code .` 명령어를 실행하여 VSCode를 열고 `templates` 폴더를 생성할 수 있습니다.

그 다음 현재 프로젝트에 [Askama](https://github.com/djc/askama)를 추가합니다.

```sh:
$ cargo add askama
```

Rust 개발 환경에서 HTMX 템플릿을 지원하기 위해 만들어진 라이브러리라고 이해하시면 됩니다.

이제 프로젝트의 디렉토리 구조는 다음과 같습니다.

```Output:
myapp
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
├── examples
├── migration
├── src
├── templates       # HTMX 템플릿 파일 저장 위치 !!!
└── tests
```

### 2.2 HTML 파일 작성
HTML 고수라면 HTML 만으로도 멋진 페이지를 개발할 수 있습니다. [CSS](https://namu.wiki/w/CSS)라고 알려진 스타일시트 언어를 잘 구사하면 HTML 파일은 단순하게 만들면서도 디자인과 기능성이 뛰어난 웹 페이지를 만들 수 있습니다. 하지만 화면이나 화면의 구성요소가 추가되면서 CSS의 구조 자체가 복잡해지고 관리가 어려워지는 문제가 있습니다. [Tailwind CSS](https://tailwindcss.com/)는 CSS의 복잡성 문제를 해결해 주는 좋은 대안압니다.

HTML 문서를 만드는 데 익숙하지 않고 Tailwind CSS는 고사하고 스타일시트 언어에 대해서도 생소하다구요? 걱정하지 마세요. 뛰어난 전문가들이 잘 만들어 놓은 템플릿을 가져와 자신만의 웹 페이지로 조금씩 바꾸는 방법이 있습니다.

이 장에서 우리가 만들려는 프론트엔드는 내부 관리자를 위한 웹 애플리케이션입니다. 인터넷 검색을 통해 찾아낸 [Tailwind CSS](https://tailwindcss.com/)를 이용한 관리자 앱을 위한 무료 템플릿 [Tailwind Admin Template](https://github.com/davidgrzyb/tailwind-admin-template)을 이용해 봅시다. 

깃헙에서 위 프로젝트의 압축파일을 [Download](https://github.com/davidgrzyb/tailwind-admin-template/archive/refs/heads/master.zip)한 후 압축을 푸신 후, 모든 HTML 파일을 우리 프로젝트의 `templates` 폴더에 복사하여 붙여 넣습니다.

복사해 온 HTML 파일의 내용을 잠깐 살펴 보시면 알겠지만, 위 템플릿에서 메인 화면에 해당하는 것은 index.html 파일입니다. 다음 절에서는 웹브라우저에서 HTTP 요청을 보내 이 페이지를 띄우는 방법에 대해 알아보겠습니다.

### 2.3 웹페이지 로딩을 위한 라우팅 설정

웹 서버는 클라이언트에서 오는 여러가지 HTTP 요청에 대해 그 요청에 포함되어 있는 URL을 통해 각각 어떤 결과를 반환해야 하는지(또는 어떤 페이지를 로딩해야 하는지)를 결정합니다. URL은 쉽게 말하면 웹브라우저의 주소창에 입력하는 소위 '주소'에 해당합니다. 주소 또는 URL은 프로토콜(protocol), 도메인(domain), 패스(path)로 구성되는데, 예를 들어, `http://acme.com/api/articles`에서 `http`는 프로토콜, `acme.com`은 도메인, `api/articles`는 패스에 해당합니다. 프로토콜은 디폴트값이 `http`이기 떄문에 생략 가능합니다. 또다른 URL `localhost:3000/api/articles`에서 프로토콜은 `http`(생략), 도메인은 `localhost:3000`(3000은 통신 포트 번호. 생략하면 디폴트 값이 80)이며, 패스는 앞의 예와 동일합니다.

프로토콜과 도메인 정보를 통해 클라이언트의 요청이 서버에 도달하면, 서버는 패스 정보를 통해 어떤 행동을 취할지(요청에 대해 어떤 로직을 수행하여 그 결과로서 어떤 응답을 클라이언트에게 보내야 하는지)를 판단하게 됩니다. 웹 서버가 어떤 패스에 어떻게 응답해야 하는지를 정해 놓은 것을 라우팅(routing)이라고 합니다.

[Loco](https://loco.rs) 프레임웍에서 라우팅에 대한 정보는 `src/app.rs`에 설정되어 있습니다. 파일을 열어 `routes()` 함수 부분을 확인해 보세요.

```rust:
fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::article::routes())
            .prefix("/api")
            .add_route(controllers::notes::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
    }
```

`cargo loco start` 명령으로 웹 서버가 시작하면서 위 함수가 실행됩니다. `.add_route(controllers::article::routes())`와 `.prefix("api")`를 눈여겨 보시기 바랍니다. 우리가 1장에서 스캐폴딩한 후 별도의 추가 코딩 없이 `localhost:3000/api/articles`라는 URL을 이용해 게시물을 추가하거나 조회할 수 있었던 것은, 스캐폴딩 과정에서 `.add_route(controllers::article::routes())` 부분이 자동으로 추가(injected)되었기 때문입니다. 덧붙여 설명하자면, `.prefix("api")`는 `loco new` 명령으로 `myapp` 프로젝트를 생성할 때 우리가 `Rest API`를 선택했기 때문인데, 만약 `SaaS App`을 선택했다면 이 부분이 생략됩니다.

이번에는 `src/controllers/articles.rs` 파일을 열어 보겠습니다. `fn routes()` 함수 부분을 확인해 보세요.

```rust:
pub fn routes() -> Routes {
    Routes::new()
        .prefix("articles")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
```

1장 마지막 절에서 CRUD 테스트에 사용한 요청 형식과 패스는 CREATE: (POST, /api/articles), READ: (GET, /api/articles & /api/articles/1), UPDATE: (POST, /api/articles/1), DELETE: (DELETE, /api/articles/1)이었던 것을 기억하나요? 위 `routes()` 함수는 이 요청들을 각각 add, list & get_one, update, remove 함수에 매핑해 주고 있습니다.

서버가 백엔드 API 뿐만 아니라 프론트엔드 페이지에 대한 요청까지 처리할 수 있게 하려면 현재의 라우팅 설정을 바꿔줘야 합니다.

우선 현재 서버는 패스에 `/api`가 있는 요청만 처리하도록 되어 있는데, `/admin`이라는 접두사를 가진 패스를 가진 요청을 받으면 관리자용 웹 페이지를 띄워주도록 바꿔 봅시다.

`src/app.rs`의 `rountes()` 함수에서 `.prefix("api")` 부분을 삭제합니다.

```rust:
    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::article::routes())
            .add_route(controllers::notes::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
    }
```

`src/controllers/articles.rs`의 `routes()` 함수의 `prefix("articles")`를 `prefix("api/articles")`라고 고쳐주세요.(현재 이미 만들어져 있는 모든 컨트롤러 - `src/controllers` 폴더에 있는 `auth.rs`, `notes.rs`, `user.rs` - 에 대해서 똑같이 해주세요.)

```rust:
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/articles")    // 'api/' added
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
```

관리자용 웹 페이지에 대한 요청은 별도의 컨트롤러에서 처리할 수 있도록 `src/controllers/admin` 폴더를 만들고 그 아래에 `home.rs` 파일과 `mod.rs` 파일을 만드세요.
디렉토리 구조는 아래와 같이 바뀌어 있을 것입니다.
```Output:
├── src
│ 	 ├── app.rs               # 변경 필요
│ 	 └── controllers
│ 	   	 ├── admin
│ 	   	 │    ├── home.rs     # 새로 추가
│ 	   	 │    └── mod.rs      # 새로 추가
│ 	   	 ├── articles.rs
│ 	   	 ├── auth.rs
│ 	   	 ├── mod.rs           # 변경 필요
│ 	   	 ├── notes.rs
│ 	   	 └── user.rs
└── templates
  	 ├── blank.html           # 변경 필요
  	 ├── calendar.html        # 변경 필요
  	 ├── forms.html           # 변경 필요
  	 ├── index.html           # 변경 필요
  	 ├── tables.html          # 변경 필요
  	 └── tabs.html            # 변경 필요
```

우선 템플릿으로 사용할 모든 html 파일에서 `<a href=` 부분의 html 파일에 대한 링크를 서버 패스 형식으로 수정합니다. 즉, "index.html"는 "index", "blank.html"는 "blank", "calendar.html"는 "calendar", "forms.html"는 "forms", "tables.html"는 "tables", "tabs.html"는 "tabs"로 일괄 수정합니다.

우리의 계획은 `home.rs`에서 각 요청에 대하여 대응하는 html 파일을 템플릿으로 써서 웹 페이지를 렌더링하도록 하려는 것입니다. `src/controllers/admin/home.rs` 파일의 소스코드는 아래와 같습니다.

```rust:
use loco_rs::prelude::*;
use askama::Template;
use axum::response::Html;
use axum::debug_handler;

#[derive(Template)]
#[template(path="blank.html")]
pub struct BlankTemplate {}

#[derive(Template)]
#[template(path="calendar.html")]
pub struct CalnedarTemplate {}

#[derive(Template)]
#[template(path="forms.html")]
pub struct FormsTemplate {}

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path="tables.html")]
pub struct TablesTemplate {}

#[derive(Template)]
#[template(path="tabs.html")]
pub struct TabsTemplate {}

#[debug_handler]
pub async fn blank() -> Result<Response> {
    let template = BlankTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[debug_handler]
pub async fn calendar() -> Result<Response> {
    let template = CalnedarTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[debug_handler]
pub async fn forms() -> Result<Response> {
    let template = FormsTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[debug_handler]
pub async fn index() -> Result<Response> {
    let template = IndexTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[debug_handler]
pub async fn tables() -> Result<Response> {
    let template = TablesTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[debug_handler]
pub async fn tabs() -> Result<Response> {
    let template = TabsTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/", get(index))
        .add("/index", get(index))
        .add("/forms", get(forms))
        .add("/tables", get(tables))
        .add("/blank", get(blank))
        .add("/calendar", get(calendar))
        .add("/tabs", get(tabs))
}
```

서버가 새로 추가된 경로를 처리할 수 있도록 다음과 같이 각 파일의 소스 코드를 수정합니다.
`src/controllers/admin/mod.rs`에 다음의 한 줄을 추가합니다.

```rust:
pub mod home;
```

`src/controllers/mod.rs`을 다음과 같이 변경합니다.

```rust:
pub mod auth;
pub mod notes;
pub mod user;

pub mod article;
pub mod admin;        // 추가
```

`src/app.rs`의 `routes()`를 다음과 같이 변경합니다.

```rust:
    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::article::routes())
            .add_route(controllers::notes::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::admin::home::routes())   // 추가
    }
```

서버를 다시 시작합니다.(혹시 서버가 실행중이라면 해당 쉘에서 Ctrl+C 를 눌러 종료한 후 다시 `cargo loco start` 명령을 실행합니다.)

서버가 시작하면 웹 브라우저 주소창에 `localhost:3000`을 입력하고 엔터키를 칩니다.
아래와 같은 화면이 나타나면 성공!
![Index Page Loaded](assets/static/htmx-admin-template-loaded.png)

화면 왼쪽의 Dashboard, Balnk Page,Tables, Forms, Tabbed Content, Calendar를 하나씩 클릭하여 정상적으로 동작하는지 확인해 보세요.

### 2.4 템플릿과 MVC 연동하기

이번에는 `articles`에 대한 템플릿을 만들어 이를 백엔드 서버의 MVC와 연동해 봅시다. 예제로 활용할 수 있는 html 템플릿을 최대한 활용하겠습니다.

`templates/blank.html` 파일을 복사하여 새로 `templates/articles.html` 파일을 만든 후 다음과 같이 파일 내용을 수정해 주세요. 참고로 아래 파일의 입력 양식(폼)과 표(테이블)은 `forms.html` 과 `tables.html`에 있는 것을 복사하여 수정하였습니다.

```HTML:
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Admin Panel</title>
    <meta name="author" content="Kibong Moon">
    <meta name="description" content="Modified from Tailwind Admin Template by David Grzyb">

    <!-- Tailwind -->
    <link href="https://cdnjs.cloudflare.com/ajax/libs/tailwindcss/2.2.19/tailwind.min.css" rel="stylesheet">
    <style>
        @import url('https://fonts.googleapis.com/css?family=Karla:400,700&display=swap');
        .font-family-karla { font-family: karla; }
        .bg-sidebar { background: #3d68ff; }
        .cta-btn { color: #3d68ff; }
        .upgrade-btn { background: #1947ee; }
        .upgrade-btn:hover { background: #0038fd; }
        .active-nav-link { background: #1947ee; }
        .nav-item:hover { background: #1947ee; }
        .account-link:hover { background: #3d68ff; }
    </style>
</head>
<body class="bg-gray-100 font-family-karla flex">

    <aside class="relative bg-sidebar h-screen w-64 hidden sm:block shadow-xl">
        <div class="p-6">
            <a href="index" class="text-white text-3xl font-semibold uppercase hover:text-gray-300">Admin</a>
            <button class="w-full bg-white cta-btn font-semibold py-2 mt-5 rounded-br-lg rounded-bl-lg rounded-tr-lg shadow-lg hover:shadow-xl hover:bg-gray-300 flex items-center justify-center">
                <i class="fas fa-plus mr-3"></i> New Report
            </button>
        </div>
        <nav class="text-white text-base font-semibold pt-3">
            <a href="index" class="flex items-center text-white opacity-75 hover:opacity-100 py-4 pl-6 nav-item">
                <i class="fas fa-tachometer-alt mr-3"></i>
                Dashboard
            </a>
            <a href="articles" class="flex items-center active-nav-link text-white py-4 pl-6 nav-item">
                <i class="fas fa-sticky-note mr-3"></i>
                Articles
            </a>
            <a href="tables" class="flex items-center text-white opacity-75 hover:opacity-100 py-4 pl-6 nav-item">
                <i class="fas fa-table mr-3"></i>
                Tables
            </a>
            <a href="forms" class="flex items-center text-white opacity-75 hover:opacity-100 py-4 pl-6 nav-item">
                <i class="fas fa-align-left mr-3"></i>
                Forms
            </a>
            <a href="tabs" class="flex items-center text-white opacity-75 hover:opacity-100 py-4 pl-6 nav-item">
                <i class="fas fa-tablet-alt mr-3"></i>
                Tabbed Content
            </a>
            <a href="calendar" class="flex items-center text-white opacity-75 hover:opacity-100 py-4 pl-6 nav-item">
                <i class="fas fa-calendar mr-3"></i>
                Calendar
            </a>
        </nav>
    </aside>

    <div class="relative w-full flex flex-col h-screen overflow-y-hidden">
        <!-- Desktop Header -->
        <header class="w-full items-center bg-white py-2 px-6 hidden sm:flex">
            <div class="w-1/2"></div>
            <div x-data="{ isOpen: false }" class="relative w-1/2 flex justify-end">
                <button @click="isOpen = !isOpen" class="realtive z-10 w-12 h-12 rounded-full overflow-hidden border-4 border-gray-400 hover:border-gray-300 focus:border-gray-300 focus:outline-none">
                    <img src="https://source.unsplash.com/uJ8LNVCBjFQ/400x400">
                </button>
                <button x-show="isOpen" @click="isOpen = false" class="h-full w-full fixed inset-0 cursor-default"></button>
                <div x-show="isOpen" class="absolute w-32 bg-white rounded-lg shadow-lg py-2 mt-16">
                    <a href="#" class="block px-4 py-2 account-link hover:text-white">Account</a>
                    <a href="#" class="block px-4 py-2 account-link hover:text-white">Support</a>
                    <a href="#" class="block px-4 py-2 account-link hover:text-white">Sign Out</a>
                </div>
            </div>
        </header>

        <!-- Mobile Header & Nav -->
        <header x-data="{ isOpen: false }" class="w-full bg-sidebar py-5 px-6 sm:hidden">
            <div class="flex items-center justify-between">
                <a href="index" class="text-white text-3xl font-semibold uppercase hover:text-gray-300">Admin</a>
                <button @click="isOpen = !isOpen" class="text-white text-3xl focus:outline-none">
                    <i x-show="!isOpen" class="fas fa-bars"></i>
                    <i x-show="isOpen" class="fas fa-times"></i>
                </button>
            </div>

            <!-- Dropdown Nav -->
            <nav :class="isOpen ? 'flex': 'hidden'" class="flex flex-col pt-4">
                <a href="index" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-tachometer-alt mr-3"></i>
                    Dashboard
                </a>
                <a href="articles" class="flex items-center active-nav-link text-white py-2 pl-4 nav-item">
                    <i class="fas fa-sticky-note mr-3"></i>
                    Articles
                </a>
                <a href="tables" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-table mr-3"></i>
                    Tables
                </a>
                <a href="forms" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-align-left mr-3"></i>
                    Forms
                </a>
                <a href="tabs" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-tablet-alt mr-3"></i>
                    Tabbed Content
                </a>
                <a href="calendar" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-calendar mr-3"></i>
                    Calendar
                </a>
                <a href="#" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-cogs mr-3"></i>
                    Support
                </a>
                <a href="#" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-user mr-3"></i>
                    My Account
                </a>
                <a href="#" class="flex items-center text-white opacity-75 hover:opacity-100 py-2 pl-4 nav-item">
                    <i class="fas fa-sign-out-alt mr-3"></i>
                    Sign Out
                </a>
            </nav>
        </header>
    
        <div class="w-full h-screen overflow-x-hidden border-t flex flex-col">
            <main class="w-full flex-grow p-6">
                <h1 class="w-full text-3xl text-black pb-6">Articles</h1>

                <div class="flex flex-wrap">
                    <div class="w-full my-6 pr-0 lg:pr-2">
                        <p class="text-xl pb-6 flex items-center">
                            <i class="fas fa-list mr-3"></i> Search Form
                        </p>
                        <div class="leading-loose">
                            <form class="p-10 bg-white rounded shadow-xl flex flex-col">
                                <div class="flex flex-row">
                                    <div class="inline-block lg:w-1/3">
                                        <label class="text-sm text-gray-600" for="title">Title</label>
                                        <input class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded" id="title" name="title" type="text" placeholder="Article Title" aria-label="Title">
                                    </div>
                                    <div class="inline-block lg:w-2/3 ml-2">
                                        <label class="text-sm text-gray-600" for="content">Content</label>
                                        <input class="w-full px-5  py-1 text-gray-700 bg-gray-200 rounded" id="content" name="content" type="text" placeholder="Article Content" aria-label="Content">
                                    </div>
                                </div>
                                <div class="inline-block mt-2">
                                    <label class="text-sm text-gray-600" for="created_at">Created At</label>
                                    <label class="text-sm text-gray-600" for="created_at_from">From</label>
                                    <input type="datetime-local" class="text-gray-700 bg-gray-200 rounded" id="created_at_from" name="created_at_from" aria-label="Created At From">
                                    <label class="text-sm text-gray-600" for="created_at_to">To</label>
                                    <input type="datetime-local" class="text-gray-700 bg-gray-200 rounded" id="created_at_to" name="created_at_to" aria-label="Created At To">
                                </div>

                                <div class="mt-6">
                                    <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit">Search</button>
                                </div>
                                <script>
                                    document.getElementById('created_at_from').value = new Date().toISOString().substring(0, 10) + "T00:00";
                                    document.getElementById('created_at_to').value = new Date().toISOString().substring(0, 10) + "T23:59";
                                </script>

                            </form>
                        </div>
                    </div>

                    <div class="w-full mt-6 pl-0 lg:pl-2">
                        <p class="text-xl pb-6 flex items-center">
                            <i class="fas fa-list mr-3"></i> Search Result
                        </p>
                        <div class="bg-white overflow-auto">
                            <table class="min-w-full bg-white">
                                <thead class="bg-gray-800 text-white">
                                    <tr>
                                        <th class="text-left py-3 px-4 uppercase font-semibold text-sm">ID</th>
                                        <th class="w-1/4 text-left py-3 px-4 uppercase font-semibold text-sm">Title</th>
                                        <th class="w-1/2 text-left py-3 px-4 uppercase font-semibold text-sm">Content</th>
                                        <th class="text-left py-3 px-4 uppercase font-semibold text-sm">Created At</td>
                                        <th class="text-left py-3 px-4 uppercase font-semibold text-sm">Updated At</td>
                                    </tr>
                                </thead>
                                <tbody class="text-gray-700">
                                    <tr>
                                        <td class="text-left py-3 px-4">1</td>
                                        <td class="text-left py-3 px-4">Hello</td>
                                        <td class="text-left py-3 px-4">World</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                    </tr>
                                    <tr class="bg-gray-200">
                                        <td class="text-left py-3 px-4">2</td>
                                        <td class="text-left py-3 px-4">Bye</td>
                                        <td class="text-left py-3 px-4">World</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                    </tr>
                                    <tr>
                                        <td class="text-left py-3 px-4">3</td>
                                        <td class="text-left py-3 px-4">Hello</td>
                                        <td class="text-left py-3 px-4">World, again</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                    </tr>
                                    <tr class="bg-gray-200">
                                        <td class="text-left py-3 px-4">4</td>
                                        <td class="text-left py-3 px-4">Bye</td>
                                        <td class="text-left py-3 px-4">World, again</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                        <td class="text-left py-3 px-4">2024-05-30 오전 12:00</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </main>
        </div>     
    </div>

    <!-- AlpineJS -->
    <script src="https://cdn.jsdelivr.net/gh/alpinejs/alpine@v2.x.x/dist/alpine.min.js" defer></script>
    <!-- Font Awesome -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.13.0/js/all.min.js" integrity="sha256-KzZiKy0DWYsnwMF+X1DvQngQ2/FxF7MF3Ff72XcpuPs=" crossorigin="anonymous"></script>
</body>
</html>
```

HTMX 템플릿을 본격적으로 사용하기 위해 Head 영역에 아래 두 줄을 추가했습니다.

```html:
    <script src="https://unpkg.com/htmx.org@latest"></script>
    <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
```

`index.html`, `calendar.html`, `forms.html`, `tables.html`, `tabs.html`의 `<body>`-`<aside>`-`<nav>` 블록에서 `href="blank"`라고 되어 있는 부분을 모두 `href="articles"`로 변경하고, `Blank Page`라고 된 텍스트를 `Articles`로 바꿔 줍니다.

새 파일 `src/controllers/admin/articles.rs`를 만들어 다음 내용을 추가합니다.

```rust:
use loco_rs::prelude::*;
use askama::Template;
use axum::response::Html;
use axum::debug_handler;

#[derive(Template)]
#[template(path="articles.html")]
pub struct PageTemplate {}

#[debug_handler]
pub async fn render() -> Result<Response> {
    let template = PageTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/articles")
        .add("/", get(render))
}
```

추가한 컨트롤러가 프로젝트에 포함되도록 `src/controllers/admin/mod.rs`에 다음 줄 추가.

```
pub mod articles;
```

이 컨트롤러를 라우팅에서 호출할 수 있도록 `src/app.rs`의 `fn routes()` 함수의 마지막에 다음 줄 추가.

```rust:
        .add_route(controllers::admin::articles::routes())
```

서버를 재시작한 후 왼쪽 네비게이션 메뉴에서 `Articles`를 클릭하여 다음 화면이 나타나면 성공!

![Articles page with dummy data](assets/static/articles-page-with-dummy-data.png)

새로 만든 화면에서 실제 데이터 조회하려면 먼저 해결해야 할 문제가 있습니다. 우리의 API는 JSON 형식의 데이터를 반환하는데 HTML 템플릿에서 직접 JSON 데이터를 접근하지 못합니다. 그런데 우리가 만드는 프론트엔드는 백엔드 서버상에서 동작하기 때문에 API를 통해 데이터를 주고받는 대신 직접 백엔드 함수를 호출할 수 있기 때문에 약간의 수정만으로 이 문제를 해결할 수 있습니다.

이를 위해 `src/controllers/article.rs`의 `list()` 함수를 리팩토링해 봅시다. 이 함수는 요청 파라메터로부터 검색 조건을 만들어 내고 데이터베이스 쿼리를 실행하고 이를 JSON 형식으로 반환하는 일을 모두 수행하고 있는데, 이를 다음과 같이 두 개의 함수로 분리합니다.

```rust:
pub async fn list_inner(ctx: &AppContext, query_params: &QueryParams) -> Result<PageResponse<Model>> {
    let title_filter = query_params.title.as_ref().unwrap_or(&String::new()).clone();
    let content_filter = query_params.content.as_ref().unwrap_or(&String::new()).clone();
    let mut condition = Condition::all();
    if !title_filter.is_empty() {
        condition = condition.add(Column::Title.contains(&title_filter));
    }
    if !content_filter.is_empty() {
        condition = condition.add(Column::Content.contains(&content_filter));
    }
    
    model::query::exec::paginate(
        &ctx.db, Entity::find(), Some(condition), &query_params.pagination_query
    ).await
}

#[debug_handler]
pub async fn list(Query(query_params): Query<QueryParams>, State(ctx): State<AppContext>) -> Result<Response> {
    let response = list_inner(&ctx, &query_params).await?;
    format::json(PaginationResponse::response(response, &query_params.pagination_query))
}
```

그런 다음 `src/controllers/admin/articles.rs`에서는 새로 분리한 `list_inner()` 함수를 호출하여 JSON으로 변환하지 않은 데이터를 이용하여 HTML의 내용을 채울 수 있게 다음과 같은 코드를 추가해 줍니다.

```rust:
use loco_rs::prelude::*;
use askama::Template;
use axum::response::Html;
use axum::debug_handler;

use loco_rs::model::query::PageResponse;
use loco_rs::controller::Json;
use axum::extract::Query;
use query::PaginationQuery;

use crate::controllers::article;
use crate::controllers::article::QueryParams;
use crate::models::_entities::articles::Model;
use std::cmp;

pub struct ItemTemplate {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
impl ItemTemplate {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            title: item.title.unwrap_or_default(),
            content: item.content.unwrap_or_default(),
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Template)]
#[template(path = "components/article_table.html")]
pub struct ListTemplate {
    pub rows: Vec<ItemTemplate>,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl ListTemplate {
    const fn new() -> Self {
        Self {
            rows: Vec::<ItemTemplate>::new(),
            page: 0,
            page_size: 0,
            total_pages: 0,
        }
    }
    fn add(&mut self, elem: ItemTemplate) {
        self.rows.push(elem);
    }
    
    fn build(response: PageResponse<Model>, pagination_query: &PaginationQuery) -> Self {
        let mut template: ListTemplate = response.page.into_iter().map(ItemTemplate::from).collect();
        template.page = pagination_query.page;
        template.page_size = pagination_query.page_size;
        template.total_pages = response.total_pages;
        template
    }
}

impl FromIterator<ItemTemplate> for ListTemplate {
    fn from_iter<U>(iter: U) -> Self
    where U: IntoIterator<Item=ItemTemplate> {
        let mut c = Self::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
///
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn list(Query(query_params): Query<QueryParams>, State(ctx): State<AppContext>) -> Result<Response> {
    let response = article::list_inner(&ctx, &query_params).await?;
    let template = ListTemplate::build(response, &query_params.pagination_query);
    
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}
```

같은 파일의 `fn routes()` 함수에 아래 라우팅을 추가합니다.

```rust:
  .add("/list", get(list))
```

이제 `templates/articles.html` 템플릿에서 양식(form)에 `id` 속성과 HTMX 속성을 추가해야 합니다.

아래와 같이 되어 있는 부분을 찾으세요.

```html:
  <form class="p-10 bg-white rounded shadow-xl flex flex-col">
```

이 부분을 아래와 같이 변경해 주세요.

```html:
  <form class="p-10 bg-white rounded shadow-xl flex flex-col" id="search-form" hx-get="/articles/list" hx-target="#search-result" hx-trigger="submit, path-deps" hx-ext="path-deps" path-deps="/api/articles" hx-swap="outerHTML">
```

그 다음 아래와 같이 되어 있는 부분을 찾으세요.

```html:
                        <p class="text-xl pb-6 flex items-center">
                            <i class="fas fa-list mr-3"></i> Search Result
                        </p>
                        <div class="bg-white overflow-auto">
                            <table class="min-w-full bg-white">
```

이 부분에서 `<div>` 요소에 `id` 속성을 아래와 같이 추가해 줍니다.

```html:
                        <p class="text-xl pb-6 flex items-center">
                            <i class="fas fa-list mr-3"></i> Search Result
                        </p>
                        <div class="bg-white overflow-auto" id="search-result">
                            <table class="min-w-full bg-white">
```

`<tbody>` 아래 요소인 `<tr>`을 보면 `class` 속성이 홀수 행에는 없고 짝수 행에만 있습니다. 데이터 행의 배경색이 흰색과 회색이 번갈아 나타나도록 하기 위함입니다. 예시로 넣은 데이터 행에는 짝수 행에 일일이 `class` 속성을 지정하면 되었는데 스타일을 이용해 자동으로 배경색을 바꿀 수 있게 하려면 HTML의 맨 위 `<head>` 부분의 `<style>` 요소에 다음의 스크립트를 추가해 줍니다.

```html:
        tbody tr:nth-child(even) {
            background-color: #e5e7eb;
          }
```

위 자바스크립트의 의미는 `<tbody>`의 자식 요소인 `<tr>`이 짝수(even) 번째일 때 배경색을 `#e5e7eb`로 하라는 의미입니다. Tailwind CSS의 `class="bg-gray-200"`와 같은 의미라고 이해하면 됩니다.

이제 예시 데이터는 이제 더 이상 필요없으므로 빈 `<tbody>` 요소만 남겨 두고 내용은 모두 삭제해 주세요.
```html:
                                <tbody class="text-gray-700">
                                </tbody>
```

조건 검색을 실행하기 전 빈 화면은 `articles.html` 파일만으로 표현될 수 있지만, 실제 서버에서 데이터를 반환하게 되면 `"search-result"` 영역의 표에 데이터가 채워져 나타나야 합니다. 위 `"search-form"` 양식에서 검색 버튼을 클릭하게 되면 양식에 입력한 검색 조건이 쿼리스트링으로 만들어져 `hx-get` 속성에 지정된 경로 `/articles/list`에 해당하는 서버 함수를 호출하게 되고, 반환된 결과는 `hx-target` 속성에 지정된 `#search-result`(현재 문서에서 `id`가 `search-result`인 문서 요소(element))에 매핑되는데, `hx-swap`에 `outerHTML`이 지정되어 있으므로 기존의 문서 요소를 대체할 것입니다.(참고로, 만약 `hx-swap` 속성에 `innerHTML`을 지정하면 대상 element의 자식 element로 추가됩니다.)

실제 데이터로 채워진 표로 `search-result` 영역을 대체하게 될 새로운 템플릿 파일인 `templates/components/article_table.html`을 아래와 같이 만들어 줍니다.

```html:
<div class="bg-white overflow-auto" id="search-result">
  <table class="min-w-full bg-white">
      <thead class="bg-gray-800 text-white">
          <tr>
              <th class="text-left py-3 px-4 uppercase font-semibold text-sm">ID</th>
              <th class="w-1/4 text-left py-3 px-4 uppercase font-semibold text-sm">Title</th>
              <th class="w-1/2 text-left py-3 px-4 uppercase font-semibold text-sm">Content</th>
              <th class="text-left py-3 px-4 uppercase font-semibold text-sm">Created At</td>
              <th class="text-left py-3 px-4 uppercase font-semibold text-sm">Updated At</td>
          </tr>
      </thead>
      <tbody class="text-gray-700">
        {% for article in rows %}
        <tr>
          <td class="text-left py-3 px-4">{{article.id}}</td>
          <td class="text-left py-3 px-4">{{article.title}}</td>
          <td class="text-left py-3 px-4">{{article.content}}</td>
          <td class="text-left py-3 px-4">{{article.created_at}}</td>
          <td class="text-left py-3 px-4">{{article.updated_at}}</td>
        </tr>
        {% endfor %}
      </tbody>
  </table>
</div>
```

이제 서버를 재시작하고 웹 브라우저에서 주소 `localhost:3000`로 이동하여 `Articles` 메뉴를 클릭한 후, 실제 저장되어 있는 데이터에 맞게 검색 조건을 설정한 후 Search 버튼을 클릭해 보시기 바랍니다.

이번에는 1장 말미에서 구현한 Pagination을 이용해 결과 화면 하단에 페이지 이동 막대줄을 추가해 봅시다.
`article_table.html` 파일에서 `<table> ... </table>` 요소 바로 다음에 아래 코드를 추가해 주세요.

```html:
  <div id="page-control" class="w-full flex-row justify-center">
    <script>
      function setPageParam(pval) {
        elem = document.getElementById("hidden-page-input");
        elem.value = pval;
        return true;
      }
    </script>
    <input id="hidden-page-input" type="hidden" name="page" value="1" form="search-form">
    {% let prange = 5 %}
    {% let section = page / prange %}
    {% let lastsec = total_pages / prange %}
    {% let seq = page % prange %}
    {% let lastseq = total_pages % prange %}

    {% if (seq == 0 && section > 0) %}
      {% if section > 1 %}
        {% let prevp = prange * (section - 2) + 1 %}
        <button class="btn page-move w-8" form="search-form" onclick='setPageParam("{{prevp}}")'> < </button>
      {% else %}
        <button class="btn page-move w-8" form="search-form" onclick='setPageParam("1")'> < </button>
      {% endif %}
      {% for i in ((prange * (section-1)) + 1)..(cmp::min(prange * section, total_pages) + 1) %}
        <input type="submit" class="btn page-move w-8" name="page" value="{{i}}" {% if i == page %}disabled{% endif %} form="search-form">
      {% endfor %}
      {% if prange * section < total_pages %}
        {% let nextp = prange * section + 1 %}
        <button class="btn page-move w-8" form="search-form" onclick='setPageParam("{{nextp}}")'> > </button>
      {% else %}
        <button class="btn page-move w-8" disabled> > </button>
      {% endif %}
    {% else %}
      {% if section > 0 %}
        {% let prevp = prange * (section - 1) + 1 %}
        <button class="btn page-move w-8" form="search-form" onclick='setPageParam("{{prevp}}")'> < </button>
      {% else %}
        <button class="btn page-move w-8" disabled> < </button>
      {% endif %}
      {% for i in ((prange * section) + 1)..(cmp::min(prange * (section + 1), total_pages) + 1) %}
        <input type="submit" class="btn page-move w-8" name="page" value="{{i}}" {% if i == page %}disabled{% endif %} form="search-form">
      {% endfor %}
      {% if prange * (section + 1) < total_pages %}
        {% let nextp = prange * (section + 1) + 1 %}
        <button class="btn page-move w-8" form="search-form" onclick='setPageParam("{{nextp}}")'> > </button>
      {% else %}
        <button class="btn page-move w-8" disabled> > </button>
      {% endif %}
    {% endif %}
  </div>
```

서버를 재시작한 후 웹브라우저에서 게시물 화면을 띄워 보세요. 아까처럼 홈에서 네비게이션 메뉴를 클릭하거나 주소창에 바로 `localhost:3000/articles`를 입력해 보세요.

마지막으로 생성일을 검색 조건에 추가해 봅시다. 웹 페이지의 검색 조건 입력 양식에는 이미 생성일(Created At) 구간을 조건으로 설정할 수 있게 되어 있습니다만 사실 아직 서버 쪽에는 이를 처리하는 부분이 없기 때문입니다.

다시 `src/controllers/article.rs` 파일을 엽니다.

우선 날짜 형식의 데이터를 처리할 수 있도록 `NaiveDate` 타입을 추가해 줍니다.

```rust:
use chrono::NaiveDate;
```

그 다음, `QueryParams` 구조체에 다음과 같이 검색 기간의 시작(`created_at_from`)과 끝(`created_at_to`)을 추가해 줍니다.

```rust:
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at_from: Option<String>,        // added here
    pub created_at_to: Option<String>,          // added here
    #[serde(flatten)]
    pub pagination_query: PaginationQuery,
}
```

마지막으로 `list_inner()` 함수에서 `model::query::paginate(` 바로 위에 아래 소스 코드를 추가해 주세요.
```rust:
    let created_at_from_filter = query_params.created_at_from.as_ref().unwrap_or(&String::new()).clone();
    let created_at_to_filter = query_params.created_at_to.as_ref().unwrap_or(&String::new()).clone();
    if !created_at_from_filter.is_empty() {
        let parsed = NaiveDate::parse_from_str(&created_at_from_filter, "%Y-%m-%dT%H:%M");
        match parsed {
            Ok(dt) => {
                condition = condition.add(Column::CreatedAt.gte(dt));
            },
            Err(err) => {eprint!("{}", err)},
        }        
    }
    
    if !created_at_to_filter.is_empty() {
        let parsed = NaiveDate::parse_from_str(&created_at_to_filter, "%Y-%m-%dT%H:%M");
        match parsed {
            Ok(dt) => {
                condition = condition.add(Column::CreatedAt.lte(dt));
            },
            Err(err) => {eprint!("{}", err)},
        }
    }
```

서버를 재시작한 후 웹 브라우저에서 검색기간을 바꿔가면서 날짜 조건 검색이 잘 되는지 확인해 보세요.

### 2.5 편집 기능 추가하기

Article 페이지에서 특정 게시물을 선택해서 수정 또는 삭제할 수 있는 기능을 추가해 봅시다.
`templates/components/article_table.html` 파일에서 `<tbody>` 부분을 아래와 같이 수정해 주세요.

```html:
<tbody class="text-gray-700">        
        {% for article in rows %}        
        <tr>
          <td class="text-left py-3 px-4">{{article.id}}</td>
          <td class="text-left py-3 px-4">{{article.title}}</td>
          <td class="text-left py-3 px-4">{{article.content}}</td>
          <td class="text-left py-3 px-4">{{article.created_at}}</td>
          <td class="text-left py-3 px-4">{{article.updated_at}}</td>
          <td>
            <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded"
              hx-get="/articles/{{article.id}}/edit"
              hx-target="#editor" hx-swap="outerHTML">
                Edit
            </button>
            <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded"
              hx-delete="/api/articles/{{ article.id }}"
              hx-target="closest tr" hx-swap="outerHTML"
              hx-confirm="Are you sure you want to delete this article?">
                Delete
            </button>
          </td>
        </tr>        
        {% endfor %}
      </tbody>
```

`Edit` 버튼을 클릭하면 `hx-target`에 지정된대로 `id`가 `editor`인 문서 요소를 대체할 것입니다. 기존 `articles.html`에서 `<main>` 요소 아래 끝에 `editor` 요소를 추가해 줍시다. 같은 위치에서 기존 데이터에 대한 수정과 신규 데이터 작성이 가능하도록 만들기 위해 `editor` 요소 안에 새 게시물 작성을 위한 버튼도 추가해 줍시다.

```html:
                <div class="w-full mt-6 pl-0 lg:pl-2" id="editor">
                    <div class="mt-6">
                        <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit" hx-get="/articles/new" hx-target="#editor" hx-swap="outerHTML">New</button>
                    </div>
                </div>
            </main>
```

이제 `templates/components/article_form_edit.html`을 새로 만들어 아래와 같이 작성해 줍니다.

```html:
<div class="w-full mt-6 pl-0 lg:pl-2" id="editor">
    <div class="mt-6">
        <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit" hx-get="/articles/new" hx-target="#editor" hx-swap="outerHTML">New</button>
    </div>
    <p class="text-xl pb-6 flex items-center">
        <i class="fas fa-list mr-3"></i> Edit Form
    </p>
    <div class="leading-loose">
        <form class="p-10 bg-white rounded shadow-xl" id="edit-form" hx-put="/articles/{{id}}" hx-target="#editor" hx-swap="outerHTML" hx-ext="json-enc">
            <p class="text-lg text-gray-800 font-medium pb-4">Article Details</p>
            <div class="">
                <label class="block text-sm text-gray-600" for="id">ID</label>
                <input class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded" id="id" name="id" value="{{id}}" type="text" aria-label="ID" disabled>
            </div>
            <div class="">
                <label class="block text-sm text-gray-600" for="title">Title</label>
                <input class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded" id="title" name="title" value="{{title}}" type="text" required="" placeholder="Article Title" aria-label="Title">
            </div>
            <div class="mt-2">
                <label class="block text-sm text-gray-600" for="content">Content</label>
                <input class="w-full px-5  py-4 text-gray-700 bg-gray-200 rounded" id="content" name="content" value="{{content}}" type="text" required="" placeholder="Article Content" aria-label="Content">
            </div>
            <div class="mt-2 flex lg:flex-row lg:space-x-4">
                <div class="w-full flex flex-col">
                    <label class="text-sm text-gray-600">Created At</label>
                    <span class="w-full px-1 py-1 text-gray-500 bg-gray-200 rounded">{{created_at}}</span>
                </div>
                <div class="w-full flex flex-col">
                    <label class="text-sm text-gray-600">Updated At</label>
                    <span class="w-full px-1 py-1 text-gray-500 bg-gray-200 rounded">{{updated_at}}</span>
                </div>
            </div>
            <div class="mt-6">
                <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit">Save</button>
            </div>
        </form>
    </div>
</div>
```

이번에는 신규 데이터 입력을 위한 템플릿 파일 `templates/components/article_form_new.html`을 다음과 같이 작성해 줍니다.

```html:
<div class="w-full mt-6 pl-0 lg:pl-2" id="editor">
    <div class="mt-6">
        <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit" hx-get="/articles/new" hx-target="#editor" hx-swap="outerHTML">New</button>
    </div>
    <p class="text-xl pb-6 flex items-center">
        <i class="fas fa-list mr-3"></i> New Form
    </p>
    <div class="w-full leading-loose">
        <form class="p-10 bg-white rounded shadow-xl" id="edit-form" hx-post="/articles" hx-target="#editor" hx-swap="outerHTML" hx-ext="json-enc">
            <p class="text-lg text-gray-800 font-medium pb-4">Article Details</p>
            <div class="">
                <label class="block text-sm text-gray-600" for="id">ID</label>
                <input class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded" id="id" name="id" type="text" aria-label="ID" disabled>
            </div>
            <div class="">
                <label class="block text-sm text-gray-600" for="title">Title</label>
                <input class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded" id="title" name="title" type="text" required="" placeholder="Article Title" aria-label="Title">
            </div>
            <div class="mt-2">
                <label class="block text-sm text-gray-600" for="content">Content</label>
                <input class="w-full px-5  py-4 text-gray-700 bg-gray-200 rounded" id="content" name="content" type="text" required="" placeholder="Article Content" aria-label="Content">
            </div>
            <div class="mt-6">
                <button class="px-4 py-1 text-white font-light tracking-wider bg-gray-900 rounded" type="submit">Save</button>
            </div>
        </form>
    </div>
</div>
```

위 두 개의 템플릿은 생긴 모양은 흡사하지만, 기존 데이터 편집은 `hx-put="/articles/{{id}}"`, 신규 데이터 입력은 `hx-post="/articles"` 요청을 발생시킨다는 점과 각 `<input>` 요소에 `value`의 유무가 다릅니다.

이제 이 두 개의 템플릿을 컨트롤러에 연결해 보겠습니다.
`src/controllers/article.rs`에 대해 다음과 같이 수정합니다.
`list()`에서 `list_inner()`를 분리해 낸 것과 마찬가지로 `get_one()`, `add()`, `update()`로부터 JSON으로 변환하기 전의 데이터를 반환해주는 함수를 분리해 내려고 합니다.

특정 `id` 값을 가진 게시물 하나만을 반환해 주는 `get_one()` 함수는 다음과 같이 정의 되어 있습니다.

```rust:
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}
```

이미 `load_item()` 함수가 우리가 원하는 형식의 데이터를 반환해 주는 것을 알 수 있습니다. 문제는 이 함수는 프라이빗 함수라서 같은 라이브러리(같은 파일) 안에서만 사용할 수 있다는 것입니다.

퍼블릭 함수로 변경해 주면 간단하게 이 문제를 해결할 수 있습니다. 함수 정의 첫 줄에 
`async fn load_item`이라고 되어 있는 부분을 <span style="color=blue">`pub async fn load_item`</span>이라고 바꿔 주세요.

`add()`와 `update()`는 각각 `add_inner()`와 `update_inner()`를 분리하여 아래와 같이 수정해 주세요.

```rust:
pub async fn add_inner(ctx: &AppContext, params: Params) -> Result<Model> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await;
    match item {
        Ok(v) => Ok(v),
        Err(err) => core::result::Result::Err(loco_rs::Error::DB(err)),
    }
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let item = add_inner(&ctx, params).await?;
    format::json(item)
}

pub async fn update_inner(id: i32, ctx: &AppContext, params: Params) -> Result<Model> {
    let item: Model = load_item(&ctx, id).await?;
    let mut item: ActiveModel = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await;
    match item {
        Ok(v) => Ok(v),
        Err(err) => core::result::Result::Err(loco_rs::Error::DB(err)),
    }
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = update_inner(id, &ctx, params).await?;
    format::json(item)
}
```

이제 템플릿 파일과 백엔드 컨트롤러를 결합해서 프론트엔드 기능을 구현해 봅시다. `src/controllers/admin/articles.rs` 파일에 다음을 추가해 주세요.

```rust:
#[derive(Template)]
#[template(path="components/article_form_new.html")]
pub struct NewTemplate {}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn new() -> Result<Response> {
    let template = NewTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn edit(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = article::load_item(&ctx, id).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<article::Params>) -> Result<Response> {
    let item = article::add_inner(&ctx, params).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn update(Path(id): Path<i32>, State(ctx): State<AppContext>, Json(params): Json<article::Params>) -> Result<Response> {
    let item = article::update_inner(id, &ctx, params).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}
```

`add()`와 `update()`에서 `ItemTemplate`을 템플릿으로 사용할 수 있게 하려면, `ItemTemplate` 구조체(struct) 앞에 `#[derive(Template)]` 매크로를 추가해 줘야 합니다.
`pub struct ItemTemplate {`라고 된 줄 바로 위에 아래 두 줄을 추가해 주세요.
```
#[derive(Template)]
#[template(path="components/article_form_edit.html")]
```

> [!IMPORTANT]
> Rust에서 주석(코멘트)은 다른 언어와 마찬가지로 `//`를 사용합니다. `///`도 주석을 나타내는데 특수하게 문서화에 사용되는 주석을 표시합니다.
> 통상적인 컴파일과 실행에서는 문제가 되지 않지만 깃헙에 올리게 되면 CI/CD 프로세스가 자동으로 실행되면서 코드 내용을 검사하는 Clippy가 실행되는데 런타임에 문제가 될 만한 부분에 대해 명시적으로 표시하지 않은 코드는 검사를 통과하지 못하게 됩니다. 그런 경우는 문서화 주석에 `# Panics`나 `# Errors`를 기재해 줘야 통과할 수 있습니다.

마지막으로 라우팅 설정 함수를 다음과 같이 변경합니다.

```rust:
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/articles")
        .add("/", get(render))
        .add("/list", get(list))
        .add("/new", get(new))
        .add("/:id/edit", get(edit))
        .add("/:id", put(update))
        .add("/", post(add))
}
```

이제 서버를 재시작해 보세요. 웹 브라우저에서 실제 데이테 조회 결과에서 하나를 골라 `Edit` 버튼을 클릭해 보세요. 아래와 같은 화면이 나타날 것입니다.
![Image of an all in one page for articles](assets/static/all-in-one-page-for-articles.png)

여기까지 잘 따라하셨나요? 최고👍예요!
백엔드 API 서버부터 간단한 프론트엔드 애플리케이션 개발까지 전 과정을 둘러봤습니다. 이제 배운 것을 실전에 활용해 보시기 바랍니다.