import { Button, Card, Checkbox, Form, Input, Typography } from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import './styles.css';

export default function Auth() {
  return (
    <div className='screen'>
      <Typography.Title level={2}>Sign in to your account</Typography.Title>
      <Card>
        <Form
          name='login'
          initialValues={{ remember: true }}
          className='form'
          // onFinish={onFinish}
        >
          <Form.Item name='username' rules={[{ required: true, message: 'Please input your Username!' }]}>
            <Input prefix={<UserOutlined />} placeholder='Username' />
          </Form.Item>
          <Form.Item name='password' rules={[{ required: true, message: 'Please input your Password!' }]}>
            <Input prefix={<LockOutlined />} type='password' placeholder='Password' />
          </Form.Item>
          <Form.Item>
            <Form.Item name='remember' valuePropName='checked' noStyle>
              <Checkbox>Remember me</Checkbox>
            </Form.Item>
          </Form.Item>

          <Form.Item>
            <div className='register'>
              <Button block type='primary' htmlType='submit'>
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
