import React from 'react';
import { Flex, Spin, Typography } from 'antd';
import api from '$/utils/api';
import { useBackendStore } from '$/store/backend';

export default function BackendProvider({ children }: React.PropsWithChildren) {
  const { isAlive, setIsAlive } = useBackendStore();

  React.useEffect(() => {
    api.isAlive().then(setIsAlive);
    const timeout = setInterval(async () => {
      try {
        const isAlive = await api.isAlive();
        setIsAlive(isAlive);
      } catch {
        console.error('Failed to detect backend.');
        setIsAlive(false);
      }
    }, 3000);
    return () => {
      clearTimeout(timeout);
    };
  }, [setIsAlive]);

  if (isAlive) {
    return <>{children}</>;
  }

  return (
    <Flex
      align='center'
      vertical
      justify='center'
      style={{ height: '100vh', maxWidth: 480, margin: '0 auto', textAlign: 'center' }}>
      <Typography.Title level={2}>Unsupported infrastructure</Typography.Title>
      <Typography.Paragraph>
        nurl requires a backend to function properly. Please serve the backend along with the frontend.
      </Typography.Paragraph>
      <Typography.Paragraph>
        <Typography.Link>See troubleshooting</Typography.Link> for more information.
      </Typography.Paragraph>
      <Flex gap='1rem' justify='center'>
        {!isAlive && (
          <>
            <Spin />
            <Typography.Paragraph type='secondary'>Waiting for backend...</Typography.Paragraph>
          </>
        )}
      </Flex>
    </Flex>
  );
}
