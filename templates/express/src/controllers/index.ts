import type { Request, Response } from 'express'

export function getAll(req: Request, res: Response) {
  res.json({ message: 'getAll' })
}

export function getOne(req: Request, res: Response) {
  res.json({ message: 'getOne', id: req.params.id })
}

export function create(req: Request, res: Response) {
  res.status(201).json({ message: 'created', body: req.body })
}

export function update(req: Request, res: Response) {
  res.json({ message: 'updated', id: req.params.id })
}

export function remove(req: Request, res: Response) {
  res.json({ message: 'deleted', id: req.params.id })
}
