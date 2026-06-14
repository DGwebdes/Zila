import type { Context } from 'hono'

export function notFound(c: Context) {
  return c.json({
    error: `Route ${c.req.method} ${c.req.url} not found`
  }, 404)
}
