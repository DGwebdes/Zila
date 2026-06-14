import { Hono } from 'hono'
import { getAll, getOne, create, update, remove } from '../controllers/index.js'

export const router = new Hono()

router.get('/',       getAll)
router.get('/:id',    getOne)
router.post('/',      create)
router.put('/:id',    update)
router.delete('/:id', remove)
