import { Button, Card, Checkbox, Form, Input, Typography } from 'antd';
import { UserOutlined, LockOutlined, EyeInvisibleFilled, EyeFilled } from '@ant-design/icons';
import './styles.css';
import { Link } from 'react-router';
import React from 'react';
import api from '$/utils/api';
import { useAuthStore } from '$/store/auth';
import { useMessage } from '$/providers/theme/theme';

type AuthForm = {
  username: string;
  password: string;
  remember: boolean;
};

export default function Auth() {
  const [loading, setLoading] = React.useState(false);
  const setToken = useAuthStore((s) => s.setToken);
  const [loginError, setLoginError] = React.useState(false);
  const [passwordVisible, setPasswordVisible] = React.useState(false);
  const [form] = Form.useForm();
  const m = useMessage();
  async function handleOnSubmit(values: AuthForm) {
    setLoading(true);
    setLoginError(false);
    setTimeout(async () => {
      try {
        const response = await api.login(values.username.trim(), values.password, values.remember);
        if (response.error != null) {
          setLoginError(true);
        } else {
          setToken(response.data.token);
          m.success('Successfully logged in');
        }
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
          form={form}
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
              <Input
                prefix={<LockOutlined />}
                type={passwordVisible ? 'text' : 'password'}
                placeholder='Password'
                suffix={
                  <Button
                    type='text'
                    icon={passwordVisible ? <EyeFilled /> : <EyeInvisibleFilled />}
                    onClick={() => setPasswordVisible((prev) => !prev)}
                    shape='circle'
                    size='small'
                  />
                }
              />
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
              {loginError && (
                <Typography.Paragraph type='danger' className='error'>
                  Invalid username or password
                </Typography.Paragraph>
              )}
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
