import React from 'react';
import { Layout, Form, Input, Button, Typography, Card, Space, Avatar, Table, Modal, Popconfirm } from 'antd';
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

const { Header, Content } = Layout;
const { Title, Text } = Typography;

type UrlItem = {
  id: string;
  original: string;
  shortened: string;
  customPath: string;
  createdAt: string;
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

  const onFinish = (values: { url: string; customPath: string }) => {
    // In a real app, this would make an API call to your backend
    const path = values.customPath || 'backendHandleThisPlz';
    const shortenedUrl = `${FRONTEND_URL}/${path}`;

    const newUrl: UrlItem = {
      id: Math.random().toString(),
      original: values.url,
      shortened: shortenedUrl,
      customPath: values.customPath || '',
      createdAt: new Date().toLocaleString(),
      clicks: 0,
    };

    setShortenedUrls([newUrl, ...shortenedUrls]);
    message.success('URL shortened successfully!');
    form.resetFields();
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
    editForm.setFieldsValue({
      original: record.original,
      customPath: record.customPath,
    });
    setIsEditModalVisible(true);
  };

  const handleEditCancel = () => {
    setIsEditModalVisible(false);
    setEditingUrl(null);
  };

  const handleEditSubmit = () => {
    editForm.validateFields().then((values) => {
      if (editingUrl) {
        const path = values.customPath || editingUrl.customPath || 'backendHandleThisPlz';
        const shortenedUrl = `${FRONTEND_URL}/${path}`;

        const updatedUrls = shortenedUrls.map((url) => {
          if (url.id === editingUrl.id) {
            return {
              ...url,
              original: values.original,
              shortened: shortenedUrl,
              customPath: values.customPath,
            };
          }
          return url;
        });

        setShortenedUrls(updatedUrls);
        setIsEditModalVisible(false);
        setEditingUrl(null);
        message.success('URL updated successfully');
      }
    });
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
      title: 'Custom Path',
      dataIndex: 'customPath',
      key: 'customPath',
      render: (text: string) => text || '-',
    },
    {
      title: 'Created',
      dataIndex: 'createdAt',
      key: 'createdAt',
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
            <Input addonBefore='https://short.en/' placeholder='my-custom-path' />
          </Form.Item>
        </Form>
      </Modal>
    </Layout>
  );
}
