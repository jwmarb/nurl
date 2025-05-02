import React from 'react';
import {
  Layout,
  Form,
  Input,
  Button,
  Typography,
  Card,
  Space,
  Avatar,
  Table,
  Modal,
  Popconfirm,
  DatePicker,
  Select,
  Radio,
  InputNumber,
} from 'antd';
import {
  LinkOutlined,
  CopyOutlined,
  LogoutOutlined,
  UserOutlined,
  EditOutlined,
  DeleteOutlined,
} from '@ant-design/icons';
import './App.css';
import { useAuthStore } from '$/store/auth';
import { useMessage } from '$/providers/theme/theme';
import dayjs from 'dayjs';

const { Header, Content } = Layout;
const { Title, Text } = Typography;
const { Option } = Select;

type UrlItem = {
  id: string;
  original: string;
  shortened: string;
  customPath: string;
  createdAt: string;
  expiresAt?: string;
  clicks: number;
};

export default function App() {
  const FRONTEND_URL = `${location.protocol}//${location.host}`;
  const [form] = Form.useForm();
  const [editForm] = Form.useForm();
  const [shortenedUrls, setShortenedUrls] = React.useState<UrlItem[]>([]);
  const [isEditModalVisible, setIsEditModalVisible] = React.useState(false);
  const setToken = useAuthStore((s) => s.setToken);
  const [editingUrl, setEditingUrl] = React.useState<UrlItem | null>(null);
  const message = useMessage();
  const [expirationType, setExpirationType] = React.useState<'never' | 'date' | 'duration'>('never');
  const [editExpirationType, setEditExpirationType] = React.useState<'never' | 'date' | 'duration'>('never');
  const [customDuration, setCustomDuration] = React.useState<boolean>(false);
  const [editCustomDuration, setEditCustomDuration] = React.useState<boolean>(false);

  const onFinish = (values: {
    url: string;
    customPath: string;
    expirationDate?: dayjs.Dayjs;
    expirationDuration?: string;
    customDurationValue?: number;
    customDurationUnit?: string;
  }) => {
    // In a real app, this would make an API call to your backend
    const path = values.customPath || 'backendHandleThisPlz';
    const shortenedUrl = `${FRONTEND_URL}/${path}`;

    let expiresAt: string | undefined;

    if (expirationType === 'date' && values.expirationDate) {
      expiresAt = values.expirationDate.format('YYYY-MM-DD HH:mm:ss');
    } else if (expirationType === 'duration' && values.expirationDuration) {
      // Calculate future date based on duration
      let futureDate = dayjs();

      if (values.expirationDuration === 'custom' && values.customDurationValue && values.customDurationUnit) {
        // Handle custom duration
        futureDate = futureDate.add(values.customDurationValue, values.customDurationUnit as any);
      } else {
        // Handle predefined durations
        const duration = values.expirationDuration;
        if (duration === '1h') futureDate = futureDate.add(1, 'hour');
        else if (duration === '24h') futureDate = futureDate.add(24, 'hour');
        else if (duration === '7d') futureDate = futureDate.add(7, 'day');
        else if (duration === '30d') futureDate = futureDate.add(30, 'day');
      }

      expiresAt = futureDate.format('YYYY-MM-DD HH:mm:ss');
    }

    const newUrl: UrlItem = {
      id: Math.random().toString(),
      original: values.url,
      shortened: shortenedUrl,
      customPath: values.customPath || '',
      createdAt: new Date().toLocaleString(),
      expiresAt,
      clicks: 0,
    };

    setShortenedUrls([newUrl, ...shortenedUrls]);
    message.success('URL shortened successfully!');
    form.resetFields();
    setExpirationType('never');
    setCustomDuration(false);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    message.info('Copied to clipboard!');
  };

  const handleLogout = () => {
    // This would contain actual logout logic in a real app
    setToken(null);
    message.success('Successfully logged out');
  };

  const handleDelete = (id: string) => {
    setShortenedUrls(shortenedUrls.filter((url) => url.id !== id));
    message.success('URL deleted successfully');
  };

  const showEditModal = (record: UrlItem) => {
    setEditingUrl(record);

    // Determine expiration type
    let initialExpirationType: 'never' | 'date' | 'duration' = 'never';
    let expirationDate = undefined;

    if (record.expiresAt) {
      initialExpirationType = 'date';
      expirationDate = dayjs(record.expiresAt);
    }

    setEditExpirationType(initialExpirationType);
    setEditCustomDuration(false);

    editForm.setFieldsValue({
      original: record.original,
      customPath: record.customPath,
      expirationDate: expirationDate,
    });

    setIsEditModalVisible(true);
  };

  const handleEditCancel = () => {
    setIsEditModalVisible(false);
    setEditingUrl(null);
    setEditExpirationType('never');
    setEditCustomDuration(false);
  };

  const handleEditSubmit = () => {
    editForm.validateFields().then((values) => {
      if (editingUrl) {
        const path = values.customPath || editingUrl.customPath || 'backendHandleThisPlz';
        const shortenedUrl = `${FRONTEND_URL}/${path}`;

        let expiresAt: string | undefined = undefined;

        if (editExpirationType === 'date' && values.expirationDate) {
          expiresAt = values.expirationDate.format('YYYY-MM-DD HH:mm:ss');
        } else if (editExpirationType === 'duration' && values.expirationDuration) {
          // Calculate future date based on duration
          let futureDate = dayjs();

          if (values.expirationDuration === 'custom' && values.customDurationValue && values.customDurationUnit) {
            // Handle custom duration
            futureDate = futureDate.add(values.customDurationValue, values.customDurationUnit as any);
          } else {
            // Handle predefined durations
            const duration = values.expirationDuration;
            if (duration === '1h') futureDate = futureDate.add(1, 'hour');
            else if (duration === '24h') futureDate = futureDate.add(24, 'hour');
            else if (duration === '7d') futureDate = futureDate.add(7, 'day');
            else if (duration === '30d') futureDate = futureDate.add(30, 'day');
          }

          expiresAt = futureDate.format('YYYY-MM-DD HH:mm:ss');
        }

        const updatedUrls = shortenedUrls.map((url) => {
          if (url.id === editingUrl.id) {
            return {
              ...url,
              original: values.original,
              shortened: shortenedUrl,
              customPath: values.customPath,
              expiresAt,
            };
          }
          return url;
        });

        setShortenedUrls(updatedUrls);
        setIsEditModalVisible(false);
        setEditingUrl(null);
        setEditExpirationType('never');
        setEditCustomDuration(false);
        message.success('URL updated successfully');
      }
    });
  };

  const handleDurationChange = (value: string) => {
    setCustomDuration(value === 'custom');
  };

  const handleEditDurationChange = (value: string) => {
    setEditCustomDuration(value === 'custom');
  };

  const columns = [
    {
      title: 'Original URL',
      dataIndex: 'original',
      key: 'original',
      ellipsis: true,
      render: (text: string) => <Text ellipsis={{ tooltip: text }}>{text}</Text>,
    },
    {
      title: 'Short URL',
      dataIndex: 'shortened',
      key: 'shortened',
      render: (text: string) => (
        <a href={text} target='_blank' rel='noopener noreferrer'>
          {text}
        </a>
      ),
    },
    {
      title: 'Created',
      dataIndex: 'createdAt',
      key: 'createdAt',
    },
    {
      title: 'Expires',
      dataIndex: 'expiresAt',
      key: 'expiresAt',
      render: (text: string) => text || 'Never',
    },
    {
      title: 'Clicks',
      dataIndex: 'clicks',
      key: 'clicks',
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: UrlItem) => (
        <Space>
          <Button
            icon={<CopyOutlined />}
            onClick={() => copyToClipboard(record.shortened)}
            type='text'
            title='Copy URL'
          />
          <Button icon={<EditOutlined />} onClick={() => showEditModal(record)} type='text' title='Edit URL' />
          <Popconfirm
            title='Are you sure you want to delete this URL?'
            onConfirm={() => handleDelete(record.id)}
            okText='Yes'
            cancelText='No'>
            <Button icon={<DeleteOutlined />} type='text' danger title='Delete URL' />
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <Layout className='layout'>
      <Header className='header'>
        <div className='logo'>nurl</div>
        <div className='user-controls'>
          <Space>
            <Avatar icon={<UserOutlined />} />
            <Text style={{ color: 'white' }}>User</Text>
            <Button type='link' icon={<LogoutOutlined />} onClick={handleLogout} style={{ color: 'white' }}>
              Logout
            </Button>
          </Space>
        </div>
      </Header>
      <Content className='content'>
        <div className='url-shortener-container'>
          <Card className='url-form-card'>
            <Title level={3}>Shorten Your URL</Title>
            <Text type='secondary'>Create short links that redirect to your original URL</Text>

            <Form form={form} name='url-shortener' layout='vertical' onFinish={onFinish} className='url-form'>
              <Form.Item
                name='url'
                label='Enter Long URL'
                rules={[
                  { required: true, message: 'Please enter a URL' },
                  { type: 'url', message: 'Please enter a valid URL' },
                ]}>
                <Input prefix={<LinkOutlined />} placeholder='https://example.com/very/long/url' />
              </Form.Item>

              <Form.Item name='customPath' label='Custom Path (Optional)' extra='Leave blank for a random short URL'>
                <Input addonBefore={FRONTEND_URL + '/'} placeholder='my-custom-path' />
              </Form.Item>

              <Form.Item label='Expiration' className='expiration-container'>
                <Radio.Group
                  value={expirationType}
                  onChange={(e) => setExpirationType(e.target.value)}
                  style={{ marginBottom: '10px' }}>
                  <Radio value='never'>Never</Radio>
                  <Radio value='date'>By Date</Radio>
                  <Radio value='duration'>By Duration</Radio>
                </Radio.Group>

                {expirationType === 'date' && (
                  <Form.Item
                    name='expirationDate'
                    rules={[{ required: true, message: 'Please select an expiration date' }]}>
                    <DatePicker
                      showTime
                      placeholder='Select expiration date and time'
                      style={{ width: '100%' }}
                      disabledDate={(current) => current && current < dayjs().startOf('day')}
                    />
                  </Form.Item>
                )}

                {expirationType === 'duration' && (
                  <>
                    <Form.Item
                      name='expirationDuration'
                      rules={[{ required: true, message: 'Please select a duration' }]}>
                      <Select placeholder='Select a duration' style={{ width: '100%' }} onChange={handleDurationChange}>
                        <Option value='1h'>1 hour</Option>
                        <Option value='24h'>24 hours</Option>
                        <Option value='7d'>7 days</Option>
                        <Option value='30d'>30 days</Option>
                        <Option value='custom'>Custom duration</Option>
                      </Select>
                    </Form.Item>

                    {customDuration && (
                      <Space style={{ width: '100%' }}>
                        <Form.Item
                          name='customDurationValue'
                          rules={[{ required: true, message: 'Required' }]}
                          style={{ marginBottom: 0, width: '100%' }}>
                          <InputNumber min={1} style={{ width: '100%' }} placeholder='Value' />
                        </Form.Item>
                        <Form.Item
                          name='customDurationUnit'
                          rules={[{ required: true, message: 'Required' }]}
                          style={{ marginBottom: 0, width: '100%', minWidth: 120 }}>
                          <Select placeholder='Unit'>
                            <Option value='minute'>Minutes</Option>
                            <Option value='hour'>Hours</Option>
                            <Option value='day'>Days</Option>
                            <Option value='week'>Weeks</Option>
                            <Option value='month'>Months</Option>
                          </Select>
                        </Form.Item>
                      </Space>
                    )}
                  </>
                )}
              </Form.Item>

              <Form.Item>
                <Button type='primary' htmlType='submit' block>
                  Shorten URL
                </Button>
              </Form.Item>
            </Form>
          </Card>

          {shortenedUrls.length > 0 && (
            <Card className='url-history-card'>
              <Title level={4}>Your Shortened URLs</Title>
              <Table
                dataSource={shortenedUrls}
                columns={columns}
                rowKey='id'
                pagination={{ pageSize: 5 }}
                scroll={{ x: true }}
              />
            </Card>
          )}
        </div>
      </Content>

      <Modal
        title='Edit URL'
        visible={isEditModalVisible}
        onCancel={handleEditCancel}
        footer={[
          <Button key='cancel' onClick={handleEditCancel}>
            Cancel
          </Button>,
          <Button key='submit' type='primary' onClick={handleEditSubmit}>
            Save
          </Button>,
        ]}>
        <Form form={editForm} layout='vertical'>
          <Form.Item
            name='original'
            label='Original URL'
            rules={[
              { required: true, message: 'Please enter a URL' },
              { type: 'url', message: 'Please enter a valid URL' },
            ]}>
            <Input prefix={<LinkOutlined />} />
          </Form.Item>
          <Form.Item name='customPath' label='Custom Path'>
            <Input addonBefore={FRONTEND_URL + '/'} placeholder='my-custom-path' />
          </Form.Item>

          <Form.Item label='Expiration'>
            <Radio.Group
              value={editExpirationType}
              onChange={(e) => setEditExpirationType(e.target.value)}
              style={{ marginBottom: '10px' }}>
              <Radio value='never'>Never</Radio>
              <Radio value='date'>By Date</Radio>
              <Radio value='duration'>By Duration</Radio>
            </Radio.Group>

            {editExpirationType === 'date' && (
              <Form.Item
                name='expirationDate'
                rules={[{ required: true, message: 'Please select an expiration date' }]}>
                <DatePicker
                  showTime
                  placeholder='Select expiration date and time'
                  style={{ width: '100%' }}
                  disabledDate={(current) => current && current < dayjs().startOf('day')}
                />
              </Form.Item>
            )}

            {editExpirationType === 'duration' && (
              <>
                <Form.Item name='expirationDuration' rules={[{ required: true, message: 'Please select a duration' }]}>
                  <Select placeholder='Select a duration' style={{ width: '100%' }} onChange={handleEditDurationChange}>
                    <Option value='1h'>1 hour</Option>
                    <Option value='24h'>24 hours</Option>
                    <Option value='7d'>7 days</Option>
                    <Option value='30d'>30 days</Option>
                    <Option value='custom'>Custom duration</Option>
                  </Select>
                </Form.Item>

                {editCustomDuration && (
                  <Space style={{ width: '100%' }}>
                    <Form.Item
                      name='customDurationValue'
                      rules={[{ required: true, message: 'Required' }]}
                      style={{ marginBottom: 0, width: '100%' }}>
                      <InputNumber min={1} style={{ width: '100%' }} placeholder='Value' />
                    </Form.Item>
                    <Form.Item
                      name='customDurationUnit'
                      rules={[{ required: true, message: 'Required' }]}
                      style={{ marginBottom: 0, width: '100%', minWidth: 120 }}>
                      <Select placeholder='Unit'>
                        <Option value='minute'>Minutes</Option>
                        <Option value='hour'>Hours</Option>
                        <Option value='day'>Days</Option>
                        <Option value='week'>Weeks</Option>
                        <Option value='month'>Months</Option>
                      </Select>
                    </Form.Item>
                  </Space>
                )}
              </>
            )}
          </Form.Item>
        </Form>
      </Modal>
    </Layout>
  );
}
