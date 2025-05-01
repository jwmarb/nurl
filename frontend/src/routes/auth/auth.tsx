import { Button, Card, Checkbox, Form, Input, Typography } from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import './styles.css';
import { Link } from 'react-router';
import React from 'react';
import api from '$/utils/api';
import { useAuthStore } from '$/store/auth';

type AuthForm = {
  username: string;
  password: string;
  remember: boolean;
};

export default function Auth() {
  const [loading, setLoading] = React.useState(false);
  const setToken = useAuthStore((s) => s.setToken);
  const [loginError, setLoginError] = React.useState(false);
  async function handleOnSubmit(values: AuthForm) {
    setLoading(true);
    setTimeout(async () => {
      try {
        const token = await api.login(values.username, values.password, values.remember);
        setToken(token.jwt);
      } catch {
        setLoginError(true);
      }
      setLoading(false);
    }, 3000);
  }
  return (
    <div className='screen'>
      <Typography.Title level={2}>Sign in to your account</Typography.Title>
      <Card>
        <Form
          name='login'
          layout='vertical'
          initialValues={{ remember: true }}
          className='form'
          onFinish={handleOnSubmit}>
          <div>
            <Form.Item
              validateStatus={loading ? 'validating' : loginError ? 'error' : undefined}
              name='username'
              label='Username'
              rules={[{ required: true, message: 'Please input your Username!' }]}>
              <Input prefix={<UserOutlined />} placeholder='Username' />
            </Form.Item>
            <Form.Item
              validateStatus={loading ? 'validating' : loginError ? 'error' : undefined}
              label='Password'
              name='password'
              rules={[{ required: true, message: 'Please input your Password!' }]}>
              <Input prefix={<LockOutlined />} type='password' placeholder='Password' />
            </Form.Item>
          </div>
          <Form.Item
            validateStatus={loading ? 'validating' : undefined}
            name='remember'
            valuePropName='checked'
            label={null}
            layout='vertical'
            noStyle>
            <Checkbox>Remember me</Checkbox>
          </Form.Item>
          <Form.Item>
            <div className='register'>
              <Button
                block
                type='primary'
                htmlType='submit'
                className='register-label'
                loading={loading}
                disabled={loading}>
                Log in
              </Button>
              <span className='register-label'>
                Don't have an account? <Link to='/auth/register'>Register here</Link>
              </span>
            </div>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
