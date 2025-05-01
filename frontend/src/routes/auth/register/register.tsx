import { Button, Card, Form, Input, Typography } from 'antd';
import { Link } from 'react-router';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import '../styles.css';

type RegisterForm = {
  username: string;
  password: string;
  confirmPassword: string;
};

export default function Register() {
  function handleOnSubmit(values: RegisterForm) {
    console.log(values);
  }
  return (
    <div className='screen'>
      <Typography.Title level={2}>Create a new account</Typography.Title>
      <Card>
        <Form
          name='register'
          layout='vertical'
          initialValues={{ agreeToTerms: false }}
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
            <Form.Item
              label='Confirm Password'
              name='confirmPassword'
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
              <Input prefix={<LockOutlined />} type='password' placeholder='Confirm Password' />
            </Form.Item>
          </div>
          <Form.Item>
            <div className='register'>
              <Button block type='primary' htmlType='submit' className='register-label'>
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
