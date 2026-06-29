import { Component, OnInit } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';

@Component({
  selector: "app-root",
  standalone: true,
  imports: [],
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"]
})
export class AppComponent implements OnInit {
  async ngOnInit() {
    // Escuta o evento do backend para saber quando o login foi bem-sucedido.
    await listen('login-successful', (event) => {
      // Recarrega a página para atualizar o estado do chat.
      window.location.reload();
    });
  }

  /** Abre a janela de login nativa da Twitch. */
  login() {
    invoke("open_login_window");
  }
}
