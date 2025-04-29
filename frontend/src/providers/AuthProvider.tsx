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

  React.useEffect(() => {
    if (!isAlive) {
      return;
    }

    api.isAuthenticated().then((isAuthenticated) => {
      if (!isAuthenticated) {
        setToken(null);
      }
    });
  }, [isAlive, navigate, setToken]);

  React.useEffect(() => {
    if (token == null) {
      navigate('/auth');
    }
  }, [navigate, token]);

  return <>{children}</>;
}
