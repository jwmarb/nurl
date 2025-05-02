import { Button, Card, Form, Input, Typography } from 'antd';
import { Link, useNavigate } from 'react-router';
import { UserOutlined, LockOutlined, EyeInvisibleFilled, EyeFilled } from '@ant-design/icons';
import '../styles.css';
import api from '$/utils/api';
import React from 'react';
import { useMessage } from '$/providers/theme/theme';

type RegisterForm = {
  username: string;
  password: string;
  confirm_password: string;
};

export default function Register() {
  const [loading, setLoading] = React.useState<boolean>(false);
  const [revealPassword, setRevealPassword] = React.useState<boolean>(false);
  const [revealConfirmPassword, setRevealConfirmPassword] = React.useState<boolean>(false);
  const [form] = Form.useForm();
  const n = useMessage();
  const navigate = useNavigate();

  function toggleRevealPassword() {
    setRevealPassword(!revealPassword);
  }
  function toggleRevealConfirmPassword() {
    setRevealConfirmPassword(!revealConfirmPassword);
  }

  async function handleOnSubmit(values: RegisterForm) {
    setLoading(true);
    const response = await api.register(values.username, values.password, values.confirm_password);
    if (response.error != null) {
      n.error(response.error);
      if (response.error != null) {
        if (response.data?.target_field) {
          form.setFields([
            {
              name: response.data.target_field,
              errors: [response.error],
            },
          ]);
        }
      }
    } else {
      n.success('Registration successful.');
      navigate('/auth');
    }
    setLoading(false);
  }
  return (
    <div className='screen'>
      <Typography.Title level={2}>Create a new account</Typography.Title>
      <Card>
        <Form name='register' layout='vertical' className='form' onFinish={handleOnSubmit} form={form}>
          <div>
            <Form.Item
              name='username'
              label='Username'
              rules={[
                () => ({
                  validator(_, value) {
                    if (!value) return Promise.reject(new Error('Username is required!'));
                    if (value.length < 3) {
                      return Promise.reject(new Error('Username must be at least 3 characters long'));
                    }
                    return Promise.resolve();
                  },
                }),
              ]}>
              <Input prefix={<UserOutlined />} placeholder='Username' />
            </Form.Item>
            <Form.Item
              label='Password'
              name='password'
              rules={[
                () => ({
                  validator(_, value) {
                    if (!value) {
                      return Promise.reject(new Error('Password is required'));
                    }
                    if (value.length < 6) {
                      return Promise.reject(new Error('Password must be at least 6 characters long'));
                    }
                    return Promise.resolve();
                  },
                }),
              ]}>
              <Input
                prefix={<LockOutlined />}
                type={revealPassword ? 'text' : 'password'}
                placeholder='Password'
                suffix={
                  <Button
                    shape='circle'
                    icon={revealPassword ? <EyeFilled /> : <EyeInvisibleFilled />}
                    type='text'
                    size='small'
                    onClick={toggleRevealPassword}
                  />
                }
              />
            </Form.Item>
            <Form.Item
              label='Confirm Password'
              name='confirm_password'
              dependencies={['password']}
              rules={[
                { required: true, message: 'Please confirm your password!' },
                ({ getFieldValue }) => ({
                  validator(_, value) {
                    if (!value || getFieldValue('password') === value) {
                      return Promise.resolve();
                    }
                    return Promise.reject(new Error('The two passwords do not match!'));
                  },
                }),
              ]}>
              <Input
                prefix={<LockOutlined />}
                type={revealConfirmPassword ? 'text' : 'password'}
                placeholder='Confirm Password'
                suffix={
                  <Button
                    shape='circle'
                    icon={revealConfirmPassword ? <EyeFilled /> : <EyeInvisibleFilled />}
                    type='text'
                    size='small'
                    onClick={toggleRevealConfirmPassword}
                  />
                }
              />
            </Form.Item>
          </div>
          <Form.Item>
            <div className='register'>
              <Button
                block
                type='primary'
                htmlType='submit'
                className='register-label'
                loading={loading}
                disabled={loading}>
                Register
              </Button>
              <span className='register-label'>
                Already have an account? <Link to='/auth'>Login here</Link>
              </span>
            </div>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
