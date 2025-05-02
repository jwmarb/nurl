import { useAuthStore } from '$/store/auth';
import { useBackendStore } from '$/store/backend';
import api from '$/utils/api';
import React from 'react';
import { useNavigate } from 'react-router';

export default function AuthProvider(props: React.PropsWithChildren) {
  const { children } = props;
  const navigate = useNavigate();
  const isAlive = useBackendStore((state) => state.isAlive);
  const setToken = useAuthStore((state) => state.setToken);
  const token = useAuthStore((state) => state.token);
  const isHydrated = useAuthStore((state) => state.isHydrated);
  const [loading, setLoading] = React.useState<boolean>(true);

  React.useEffect(() => {
    if (!isAlive) {
      return;
    }

    if (isHydrated) {
      api
        .isAuthenticated(token)
        .then((isAuthenticated) => {
          if (!isAuthenticated) {
            setToken(null);
          }
        })
        .finally(() => setLoading(false));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAlive, navigate, setToken, isHydrated]);

  React.useEffect(() => {
    if (isHydrated) {
      if (token == null) {
        navigate('/auth');
      } else {
        navigate('/');
      }
    }
  }, [navigate, token, isHydrated]);

  if (loading) {
    return null;
  }

  return <>{children}</>;
}
