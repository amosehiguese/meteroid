import { TransportProvider } from '@connectrpc/connect-query'
import { createGrpcWebTransport } from '@connectrpc/connect-web'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { RouterProvider } from 'react-router-dom'

import { authInterceptor, errorInterceptor, loggingInterceptor } from '@/lib/connectrpc'
import { env } from '@/lib/env'

import router from './router/router'
import { Toaster } from 'sonner'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
    },
  },
})

export const App: React.FC = () => {
  const transport = createGrpcWebTransport({
    baseUrl: 'http://127.0.0.1:50061',
    interceptors: [errorInterceptor, loggingInterceptor, authInterceptor],
  })
  return (
    <>
      <TransportProvider transport={transport}>
        <QueryClientProvider client={queryClient}>
          <RouterProvider router={router} />
        </QueryClientProvider>
      </TransportProvider>

      <Toaster />
    </>
  )
}
