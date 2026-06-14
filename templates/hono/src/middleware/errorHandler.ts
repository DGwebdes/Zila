import type { Context, Next } from 'hono'

export async function errorHandler(c: Context, next: Next) {
  try {
    await next()
  } catch (err) {
    console.error(err)
    return c.json({
      error: err instanceof Error ? err.message : 'Internal Server Error'
    }, 500)
  }
}
