/* eslint-disable @typescript-eslint/no-explicit-any */
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
  Tag,
} from 'antd';
import {
  LinkOutlined,
  CopyOutlined,
  LogoutOutlined,
  UserOutlined,
  EditOutlined,
  DeleteOutlined,
  CalendarOutlined,
  ClockCircleOutlined,
} from '@ant-design/icons';
import './App.css';
import { useAuthStore } from '$/store/auth';
import { useMessage } from '$/providers/theme/theme';
import dayjs from 'dayjs';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import api, { UpdateURLRequest } from '$/utils/api';
import { BACKEND_URL } from '$/utils/constants';

const { Header, Content } = Layout;
const { Title, Text } = Typography;
const { Option } = Select;

// API response type from GET /api/shorten
type ShortenedUrlResponse = {
  id: string;
  original_url: string;
  short_url: string;
  expiry_date?: string;
  created_at: string;
  updated_at: string;
  owner: string;
  redirects: number;
};

// Front-end representation of a URL item
type UrlItem = {
  id: string;
  original: string;
  shortened: string;
  customPath: string;
  createdAt: Date;
  expiresAt?: Date;
  clicks: number;
};

// Type for POST/PUT request to /api/shorten
type CreateUpdateUrlData = {
  original_url: string;
  custom_path?: string;
  expiration?: number;
};

export default function App() {
  const FRONTEND_URL = typeof location === 'undefined' ? BACKEND_URL : `${location.protocol}//${location.host}`;
  const [form] = Form.useForm();
  const [editForm] = Form.useForm();
  const [isEditModalVisible, setIsEditModalVisible] = React.useState(false);
  const [isReplaceModalVisible, setIsReplaceModalVisible] = React.useState(false);
  const setToken = useAuthStore((s) => s.setToken);
  const token = useAuthStore((s) => s.token);
  const [editingUrl, setEditingUrl] = React.useState<UrlItem | null>(null);
  const [duplicateUrl, setDuplicateUrl] = React.useState<UrlItem | null>(null);
  const [pendingUrlData, setPendingUrlData] = React.useState<CreateUpdateUrlData | null>(null);
  const message = useMessage();
  const [expirationType, setExpirationType] = React.useState<'never' | 'date' | 'duration'>('never');
  const [editExpirationType, setEditExpirationType] = React.useState<'never' | 'date' | 'duration'>('never');
  const [customDuration, setCustomDuration] = React.useState<boolean>(false);
  const [editCustomDuration, setEditCustomDuration] = React.useState<boolean>(false);
  const queryClient = useQueryClient();
  const username = React.useMemo(() => {
    if (token) {
      const base64Url = token.split('.')[1];
      const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
      const jsonPayload = decodeURIComponent(
        window
          .atob(base64)
          .split('')
          .map(function (c) {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
          })
          .join('')
      );

      return JSON.parse(jsonPayload).username;
    }

    return 'User';
  }, [token]);

  // Function to convert API response to local format
  const mapResponseToUrlItem = (url: ShortenedUrlResponse): UrlItem => ({
    id: url.id,
    original: url.original_url,
    shortened: `${FRONTEND_URL}/${url.short_url}`,
    customPath: url.short_url,
    createdAt: new Date(url.created_at),
    expiresAt: url.expiry_date ? new Date(url.expiry_date) : undefined,
    clicks: url.redirects,
  });

  // Fetch all shortened URLs
  const { data: shortenedUrls = [] } = useQuery({
    queryKey: ['shortenedUrls'],
    queryFn: async () => {
      const response = await api.getShortenedURLs();
      return response.data.map(mapResponseToUrlItem);
    },
  });

  // Create mutation
  const createUrlMutation = useMutation({
    mutationFn: async (data: CreateUpdateUrlData) => {
      const response = await api.createShortenedURL(data);
      if (response.error) {
        throw new Error(response.error);
      }
      return response.data;
    },
    onSuccess: () => {
      message.success('URL shortened successfully!');
      form.resetFields();
      setExpirationType('never');
      setCustomDuration(false);
      queryClient.invalidateQueries({ queryKey: ['shortenedUrls'] });
    },
    onError: (error) => {
      message.error('Failed to shorten URL: ' + (error instanceof Error ? error.message : 'Unknown error'));
    },
  });

  // Create/Update URL mutation
  const updateUrlMutation = useMutation({
    mutationFn: async (data: UpdateURLRequest) => {
      const response = await api.updateShortenedURL(data);
      if (response.error) {
        throw new Error(response.error);
      }
      return response.data;
    },
    onSuccess: () => {
      message.success('URL updated successfully!');
      form.resetFields();
      setExpirationType('never');
      setCustomDuration(false);
      queryClient.invalidateQueries({ queryKey: ['shortenedUrls'] });
    },
    onError: (error) => {
      message.error('Failed to update URL: ' + (error instanceof Error ? error.message : 'Unknown error'));
    },
  });

  // Delete URL mutation
  const deleteUrlMutation = useMutation({
    mutationFn: async (id: string) => {
      await api.deleteShortenedURL(id);
      return id;
    },
    onSuccess: () => {
      message.success('URL deleted successfully');
      queryClient.invalidateQueries({ queryKey: ['shortenedUrls'] });
    },
    onError: (error) => {
      message.error('Failed to delete URL: ' + (error instanceof Error ? error.message : 'Unknown error'));
    },
  });

  const calculateExpirationSeconds = (
    type: 'never' | 'date' | 'duration',
    expirationDate?: dayjs.Dayjs,
    expirationDuration?: string,
    customDurationValue?: number,
    customDurationUnit?: string
  ): number | undefined => {
    if (type === 'never') {
      return undefined;
    } else if (type === 'date' && expirationDate) {
      // Calculate seconds from now until expiration date
      return expirationDate.diff(dayjs(), 'second');
    } else if (type === 'duration' && expirationDuration) {
      let seconds = 0;

      if (expirationDuration === 'custom' && customDurationValue && customDurationUnit) {
        // Convert custom duration to seconds
        if (customDurationUnit === 'minute') seconds = customDurationValue * 60;
        else if (customDurationUnit === 'hour') seconds = customDurationValue * 3600;
        else if (customDurationUnit === 'day') seconds = customDurationValue * 86400;
        else if (customDurationUnit === 'week') seconds = customDurationValue * 604800;
        else if (customDurationUnit === 'month') seconds = customDurationValue * 2592000; // Approx
      } else {
        // Handle predefined durations
        if (expirationDuration === '1h') seconds = 3600;
        else if (expirationDuration === '24h') seconds = 86400;
        else if (expirationDuration === '7d') seconds = 604800;
        else if (expirationDuration === '30d') seconds = 2592000;
      }

      return seconds;
    }

    return undefined;
  };

  const checkForDuplicateUrl = (originalUrl: string): UrlItem | null => {
    return shortenedUrls.find((url) => url.original === originalUrl) || null;
  };

  const onFinish = (values: {
    url: string;
    customPath: string;
    expirationDate?: dayjs.Dayjs;
    expirationDuration?: string;
    customDurationValue?: number;
    customDurationUnit?: string;
  }) => {
    const expiration = calculateExpirationSeconds(
      expirationType,
      values.expirationDate,
      values.expirationDuration,
      values.customDurationValue,
      values.customDurationUnit
    );

    const urlData = {
      original_url: values.url,
      custom_path: values.customPath || undefined,
      expiration,
    };

    // Check if this URL already exists for the user
    const duplicate = checkForDuplicateUrl(values.url);
    if (duplicate) {
      // Save the data and the duplicate URL for later use
      setDuplicateUrl(duplicate);
      setPendingUrlData(urlData);
      // Show confirmation modal
      setIsReplaceModalVisible(true);
    } else {
      // No duplicate, create new URL
      createUrlMutation.mutate(urlData);
    }
  };

  const handleReplaceUrl = () => {
    if (duplicateUrl && pendingUrlData) {
      // Update existing URL instead of creating a new one
      updateUrlMutation.mutate(
        {
          id: duplicateUrl.id,
          ...pendingUrlData,
        },
        {
          onSuccess: () => {
            setIsReplaceModalVisible(false);
            setDuplicateUrl(null);
            setPendingUrlData(null);
          },
        }
      );
    }
  };

  const handleCancelReplace = () => {
    setIsReplaceModalVisible(false);
    setDuplicateUrl(null);
    setPendingUrlData(null);
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
    deleteUrlMutation.mutate(id);
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
        const expiration = calculateExpirationSeconds(
          editExpirationType,
          values.expirationDate,
          values.expirationDuration,
          values.customDurationValue,
          values.customDurationUnit
        );

        updateUrlMutation.mutate(
          {
            id: editingUrl.id,
            original_url: values.original,
            custom_path: values.customPath || undefined,
            expiration,
          },
          {
            onSuccess: () => {
              setIsEditModalVisible(false);
              setEditingUrl(null);
              setEditExpirationType('never');
              setEditCustomDuration(false);
            },
          }
        );
      }
    });
  };

  const handleAddUrlAnyways = () => {
    if (pendingUrlData) {
      createUrlMutation.mutate(pendingUrlData);
    }
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
      render: (text: string) => (
        <Text ellipsis={{ tooltip: text }} style={{ maxWidth: '8rem' }}>
          {text}
        </Text>
      ),
    },
    {
      title: 'Short URL',
      dataIndex: 'shortened',
      key: 'shortened',
      render: (text: string) => (
        <Typography.Link href={text} target='_blank' rel='noopener noreferrer'>
          {text}
        </Typography.Link>
      ),
    },
    {
      title: 'Created',
      dataIndex: 'createdAt',
      key: 'createdAt',
      render: (text: Date) => (
        <Tag icon={<CalendarOutlined />} color='default'>
          {text.toLocaleString()}
        </Tag>
      ),
    },
    {
      title: 'Expires',
      dataIndex: 'expiresAt',
      key: 'expiresAt',
      render: (text: Date) =>
        text ? (
          dayjs().isAfter(text) ? (
            <Tag icon={<ClockCircleOutlined />} color='red'>
              Expired
            </Tag>
          ) : (
            <Tag icon={<CalendarOutlined />} color='blue'>
              {text.toLocaleString() || 'Never'}
            </Tag>
          )
        ) : (
          <Tag color='default'>Never</Tag>
        ),
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
            <Text style={{ color: 'white' }}>{username}</Text>
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
                <Button type='primary' htmlType='submit' block loading={createUrlMutation.isPending}>
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
                loading={!!queryClient.isFetching({ queryKey: ['shortenedUrls'] })}
              />
            </Card>
          )}
        </div>
      </Content>

      {/* Edit Modal */}
      <Modal
        title='Edit URL'
        open={isEditModalVisible}
        onCancel={handleEditCancel}
        footer={[
          <Button key='cancel' onClick={handleEditCancel}>
            Cancel
          </Button>,
          <Button key='submit' type='primary' onClick={handleEditSubmit} loading={updateUrlMutation.isPending}>
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

      {/* Replace URL Modal */}
      <Modal
        title='URL Already Exists'
        open={isReplaceModalVisible}
        onCancel={handleCancelReplace}
        footer={[
          <Button key='cancel' onClick={handleCancelReplace}>
            Cancel
          </Button>,
          <Button key='replace' type='primary' onClick={handleReplaceUrl} loading={updateUrlMutation.isPending}>
            Replace Existing URL
          </Button>,
          ...(duplicateUrl?.customPath !== pendingUrlData?.custom_path
            ? [
                <Button
                  key='add'
                  type='primary'
                  ghost
                  onClick={handleAddUrlAnyways}
                  loading={createUrlMutation.isPending}>
                  Add anyways
                </Button>,
              ]
            : []),
        ]}>
        <p>You already have a shortened URL for this original URL:</p>
        {duplicateUrl && (
          <div style={{ margin: '15px 0' }}>
            <p>
              <strong>Original URL:</strong> {duplicateUrl.original}
            </p>
            <p>
              <strong>Short URL:</strong> {duplicateUrl.shortened}
            </p>
          </div>
        )}
        <p>Would you like to replace the existing shortened URL with your new settings?</p>
      </Modal>
    </Layout>
  );
}
