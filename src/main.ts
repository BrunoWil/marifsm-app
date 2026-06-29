import { bootstrapApplication } from '@angular/platform-browser';
import { AppComponent } from './app/app.component';
import { ApplicationRef } from '@angular/core';

bootstrapApplication(AppComponent)
  .then((ref: ApplicationRef) => {
    // Garante que o Angular não destrua e recrie o DOM gerenciado pelo Tauri
  })
  .catch((err) => console.error(err));