import { useParams } from 'react-router-dom'

// utils to standardize the path params & simplify maintenance

export const ParamsSlugs = {
  tenantSlug: ':tenantSlug',
  organizationSlug: ':organizationSlug',
  familyExternalId: ':familyExternalId',
  planExternalId: ':planExternalId',
  planVersion: ':planVersion',
  feeType: ':feeType',
  invoiceId: ':invoiceId',
}
type AvailableParams = Record<keyof typeof ParamsSlugs, string>

type Common<A, B> = Pick<A & B, keyof A & keyof B>

export const useTypedParams = <A extends Partial<AvailableParams>>() =>
  useParams<Common<A, AvailableParams>>()
