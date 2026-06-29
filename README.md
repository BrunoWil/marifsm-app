# marifsm - Desktop App (Tauri + Angular + Bun)

Este é um aplicativo desktop ultra leve, focado exclusivamente em transmitir o canal da **marifsm** na Twitch, integrando perfeitamente o player de vídeo com o chat oficial (incluindo suporte a login e interações).

A arquitetura utiliza **Tauri v2** para renderização nativa de alta performance, **Angular** para a estrutura do frontend e **Bun** como runtime e gerenciador de pacotes ultrarrápido.

---

## 🚀 Tecnologias Utilizadas

*   **Tauri v2:** Garante um executável final minúsculo (geralmente < 10MB) utilizando a WebView nativa do sistema operacional e Rust no ecossistema de backend.
*   **Angular:** Framework frontend robusto operando em modo Standalone.
*   **Bun:** Substituição direta do Node.js, atuando como gerenciador de pacotes e executor de scripts com performance máxima.
*   **Twitch Embed API:** Solução oficial da Twitch que viabiliza o player de vídeo e o chat dinâmico em uma única view.

---

## 🛠️ Configuração do Projeto

Siga os passos abaixo para mapear e colar os códigos nos arquivos correspondentes dentro da estrutura gerada.

### 1. Script Base (`src/index.html`)
Adicione o script da Twitch Embed API dentro da tag `<head>` para disponibilizar o objeto global do player.

```html
<!doctype html>
<html lang="pt-BR">
<head>
  <meta charset="utf-8">
  <title>marifsm - Twitch</title>
  <base href="/">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    body, html {
      margin: 0;
      padding: 0;
      width: 100vw;
      height: 100vh;
      overflow: hidden;
      background-color: #0e0e10;
    }
  </style>
  <!-- Script oficial de incorporação da Twitch -->
  <script src="https://embed.twitch.tv/v1/embed.js"></script>
</head>
<body>
  <app-root></app-root>
</body>
</html>
```

### 🏃‍♂️ Como Executar
Certifique-se de ter instalado as ferramentas de build do Rust em sua máquina.

#### Instalar as dependências
```bash
bun install
```

#### Rodar em ambiente de desenvolvimento (com Live Reload)
```bash
bun tauri dev
```

#### Compilar e gerar o instalador de produção
Este comando irá gerar um instalador otimizado nativo para o seu Sistema Operacional atual (.exe, .deb ou .app):

```bash
bun tauri build
```
O binário final estará disponível no diretório: `src-tauri/target/release/bundle/`.

### 🔒 Notas de Segurança e Login
O uso do parâmetro `parent: ["tauri.localhost"]` instrui o ecossistema de autenticação da Twitch a confiar no contexto interno do app, permitindo que os usuários façam login em suas contas pessoais para enviar mensagens no chat, resgatar pontos do canal e interagir normalmente com as transmissões ao vivo da marifsm.
