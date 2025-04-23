import BackendProvider from '$/providers/BackendProvider';
import ThemeProvider from '$/providers/theme/ThemeProvider';

export default function Providers({ children }: React.PropsWithChildren) {
  return (
    <ThemeProvider>
      <BackendProvider>{children}</BackendProvider>
    </ThemeProvider>
  );
}
