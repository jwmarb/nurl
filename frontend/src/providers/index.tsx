import AuthProvider from '$/providers/AuthProvider';
import BackendProvider from '$/providers/BackendProvider';
import ThemeProvider from '$/providers/theme/ThemeProvider';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient();

export default function Providers({ children }: React.PropsWithChildren) {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <BackendProvider>
          <AuthProvider>{children}</AuthProvider>
        </BackendProvider>
      </ThemeProvider>
    </QueryClientProvider>
  );
}
