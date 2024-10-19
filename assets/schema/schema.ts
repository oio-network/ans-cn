import { z } from 'zod'

export const asnSchema = z.object({
  updated_at: z.number(),
  number: z.string(),
  name: z.string(),
  isp: z.string(),
})

export type ASN = z.infer<typeof asnSchema>
