import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import ResumeBuilder from "@/components/ResumeBuilder";
import "./globals.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ResumeBuilder />
  </StrictMode>,
);

if ("serviceWorker" in navigator) {
  if (import.meta.env.PROD) {
    window.addEventListener("load", () => {
      navigator.serviceWorker.register("/sw.js").catch((err) => {
        console.log("SW registration failed:", err);
      });
    });
  } else {
    navigator.serviceWorker.getRegistrations().then((regs) => {
      regs.forEach((r) => r.unregister());
    });
    if ("caches" in window) {
      caches.keys().then((keys) => keys.forEach((k) => caches.delete(k)));
    }
  }
}
