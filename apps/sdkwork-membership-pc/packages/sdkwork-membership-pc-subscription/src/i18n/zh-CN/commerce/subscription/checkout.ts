export const sdkworkSubscriptionCheckoutZhCnResource = {
  commerce: {
    subscription: {
      checkout: {
        activation: {
          description: "支付完成后将自动开通所选会员套餐。",
          title: "即时生效",
        },
        close: "关闭",
        completed: "支付完成",
        creatingPayment: "正在生成支付二维码...",
        payByQr: "扫码支付",
        paymentUnavailable: {
          description: "暂未获取到支付二维码，请重试。",
          title: "支付二维码不可用",
        },
        price: "价格",
        retry: "重试",
        scanPrompt: "请使用手机支付应用扫码完成支付",
        secure: {
          description: "支付信息仅用于本次订单结算。",
          title: "安全结算",
        },
        selectedPlan: "已选套餐",
      },
      dialogs: {
        close: "关闭",
        redemptionTitle: "会员兑换",
        tokenDetailsTitle: "Token 明细",
        tokenPurchaseTitle: "购买 Token",
      },
    },
  },
} as const;
