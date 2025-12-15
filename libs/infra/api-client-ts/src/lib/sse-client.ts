// libs/infra/api-client-ts/src/lib/sse-client.ts
/**
 * =================================================================
 * APARATO: SSE STREAMING CLIENT (Fetch Implementation)
 * RESPONSABILIDAD: CONSUMO DE SERVER-SENT EVENTS CON AUTH HEADERS
 * CARACTERÍSTICAS: RECONEXIÓN AUTOMÁTICA, PARSING DE CHUNKS
 * =================================================================
 */

type EventHandler = (data: any) => void;

interface SSEOptions {
  url: string;
  token?: string;
  onMessage: EventHandler;
  onError?: (err: Error) => void;
  onOpen?: () => void;
}

/**
 * Cliente de Streaming robusto que reemplaza a EventSource nativo.
 * Permite autenticación Bearer y manejo de flujos binarios.
 */
export class SSESubscription {
  private controller: AbortController;
  private retryCount = 0;
  private maxRetries = 5;
  private backoffBase = 1000;
  private isConnected = false;

  constructor(private options: SSEOptions) {
    this.controller = new AbortController();
    this.connect();
  }

  /**
   * Inicia la conexión y el bucle de lectura del stream.
   */
  private async connect() {
    try {
      const headers: HeadersInit = {
        Accept: "text/event-stream",
        "Cache-Control": "no-cache",
        Connection: "keep-alive",
      };

      if (this.options.token) {
        headers["Authorization"] = `Bearer ${this.options.token}`;
      }

      const response = await fetch(this.options.url, {
        headers,
        signal: this.controller.signal,
      });

      if (!response.ok) {
        throw new Error(
          `SSE Connection failed: ${response.status} ${response.statusText}`,
        );
      }

      if (!response.body)
        throw new Error("ReadableStream not supported in this browser");

      this.isConnected = true;
      this.retryCount = 0;
      this.options.onOpen?.();

      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";

      // Bucle de lectura de stream
      while (true) {
        const { done, value } = await reader.read();

        if (done) break;

        const chunk = decoder.decode(value, { stream: true });
        buffer += chunk;

        // Procesamiento de líneas SSE (data: {...})
        const lines = buffer.split("\n\n");
        buffer = lines.pop() || ""; // Guardar el remanente incompleto

        for (const line of lines) {
          this.parseEvent(line);
        }
      }
    } catch (error: any) {
      if (error.name === "AbortError") return; // Cierre voluntario

      this.isConnected = false;
      this.options.onError?.(error);
      this.scheduleReconnect();
    }
  }

  /**
   * Parsea un bloque de texto SSE crudo.
   * Formato esperado: "data: {json}"
   */
  private parseEvent(rawBlock: string) {
    const lines = rawBlock.split("\n");

    for (const line of lines) {
      if (line.startsWith("data: ")) {
        try {
          const jsonStr = line.substring(6);
          const data = JSON.parse(jsonStr);
          this.options.onMessage(data);
        } catch (e) {
          console.warn("[SSE] Malformed JSON chunk received", e);
        }
      }
    }
  }

  /**
   * Lógica de reconexión con Backoff Exponencial.
   */
  private scheduleReconnect() {
    if (this.retryCount >= this.maxRetries) {
      console.error("[SSE] Max retries reached. Stream terminated.");
      return;
    }

    const delay = this.backoffBase * Math.pow(2, this.retryCount);
    this.retryCount++;

    console.log(
      `[SSE] Reconnecting in ${delay}ms (Attempt ${this.retryCount})...`,
    );
    setTimeout(() => this.connect(), delay);
  }

  /**
   * Cierra la conexión y libera recursos.
   */
  public close() {
    this.controller.abort();
    this.isConnected = false;
  }
}
