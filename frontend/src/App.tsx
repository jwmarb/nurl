import React from 'react';
import { Flex, Spin, Typography } from 'antd';
import api from './utils/api';
import { CheckCircleOutlined } from '@ant-design/icons';

const App = () => {
  const [success, setSuccess] = React.useState<boolean>(false);

  React.useEffect(() => {
    if (success) {
      console.log('Detected backend');
      window.location.href = '/';
    } else {
      const timeout = setInterval(async () => {
        try {
          const isAlive = await api.isAlive();
          setSuccess(isAlive);
        } catch {
          console.error('Failed to detect backend.');
        }
      }, 3000);
      return () => {
        clearTimeout(timeout);
      };
    }
  }, [success]);
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
        {success ? (
          <>
            <Typography.Text type='success'>
              <CheckCircleOutlined />
            </Typography.Text>
            <Typography.Paragraph type='success'>Detected backend. Redirecting...</Typography.Paragraph>
          </>
        ) : (
          <>
            <Spin />
            <Typography.Paragraph type='secondary'>Waiting for backend...</Typography.Paragraph>
          </>
        )}
      </Flex>
    </Flex>
  );
};

export default App;
