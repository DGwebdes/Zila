import type { Context } from 'hono'

export function getAll(c: Context) {
  return c.json({ message: 'getAll' })
}

export function getOne(c: Context) {
  return c.json({ message: 'getOne', id: c.req.param('id') })
}

export function create(c: Context) {
  return c.json({ message: 'created' }, 201)
}

export function update(c: Context) {
  return c.json({ message: 'updated', id: c.req.param('id') })
}

export function remove(c: Context) {
  return c.json({ message: 'deleted', id: c.req.param('id') })
}
