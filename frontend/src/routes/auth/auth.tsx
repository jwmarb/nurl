import { Button, Card, Checkbox, Form, Input, Typography } from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import './styles.css';

type AuthForm = {
  username: string;
  password: string;
  remember: boolean;
};

export default function Auth() {
  function handleOnSubmit(values: AuthForm) {
    console.log(values);
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
              name='username'
              label='Username'
              rules={[{ required: true, message: 'Please input your Username!' }]}>
              <Input prefix={<UserOutlined />} placeholder='Username' />
            </Form.Item>
            <Form.Item
              label='Password'
              name='password'
              rules={[{ required: true, message: 'Please input your Password!' }]}>
              <Input prefix={<LockOutlined />} type='password' placeholder='Password' />
            </Form.Item>
          </div>
          <Form.Item name='remember' valuePropName='checked' label={null} layout='vertical' noStyle>
            <Checkbox>Remember me</Checkbox>
          </Form.Item>
          <Form.Item>
            <div className='register'>
              <Button block type='primary' htmlType='submit' className='register-label'>
                Log in
              </Button>
              <span className='register-label'>
                Don't have an account? <a href=''>Register here</a>
              </span>
            </div>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
