import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import { router } from './routes/index.js'
import { errorHandler } from './middleware/errorHandler.js'
import { notFound } from './middleware/notFound.js'
import { logger } from './middleware/logger.js'

const app = express()

app.use(helmet())
app.use(cors())
app.use(express.json())
app.use(logger)

app.use('/api', router)

// These two always go last
app.use(notFound)
app.use(errorHandler)

export default app
