import { z } from 'zod'

export const createCustomerSchema = z.object({
  companyName: z.string().min(3),
  // wrapped to simplify form handling
  primaryEmail: z.string().optional(),
  externalId: z.string().optional(),
  stripeCustomerId: z.string(),
})
