import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import { router } from './routes/index.js'

const app = express()

// Middleware
app.use(helmet())
app.use(cors())
app.use(express.json())

// Routes
app.use('/api', router)

export default app
