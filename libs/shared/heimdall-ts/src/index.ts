/**
 * HEIMDALL LOGGING SYSTEM (TypeScript Edition)
 * Estandariza la salida de logs entre el Dashboard y las Tools.
 */

type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export class Logger {
  private context: string;

  constructor(context: string) {
    this.context = context;
  }

  private emit(level: LogLevel, message: string, meta?: Record<string, any>) {
    const timestamp = new Date().toISOString();
    const isProd = process.env.NODE_ENV === 'production';

    if (isProd) {
      // Salida JSON estructurada para Vercel/Render Logs
      console.log(JSON.stringify({
        timestamp,
        level,
        context: this.context,
        message,
        ...meta
      }));
    } else {
      // Salida visual para DX (Developer Experience)
      const colors = {
        debug: '\x1b[90m', // Gris
        info: '\x1b[32m',  // Verde
        warn: '\x1b[33m',  // Amarillo
        error: '\x1b[31m', // Rojo
      };
      const reset = '\x1b[0m';

      console.log(
        `${colors[level]}[${timestamp}] [${this.context}] ${level.toUpperCase()}:${reset} ${message}`,
        meta ? meta : ''
      );
    }
  }

  debug(msg: string, meta?: any) { this.emit('debug', msg, meta); }
  info(msg: string, meta?: any) { this.emit('info', msg, meta); }
  warn(msg: string, meta?: any) { this.emit('warn', msg, meta); }
  error(msg: string, meta?: any) { this.emit('error', msg, meta); }
}

export const createLogger = (context: string) => new Logger(context);
