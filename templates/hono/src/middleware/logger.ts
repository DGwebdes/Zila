import type { Context, Next } from 'hono'

export async function logger(c: Context, next: Next) {
  console.log(`${c.req.method} ${c.req.url}`)
  await next()
}
