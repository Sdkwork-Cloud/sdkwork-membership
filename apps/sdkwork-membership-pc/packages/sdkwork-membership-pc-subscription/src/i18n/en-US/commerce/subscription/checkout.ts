export const sdkworkSubscriptionCheckoutEnUsResource = {
  commerce: {
    subscription: {
      checkout: {
        activation: {
          description: "Payment activates the selected membership automatically.",
          title: "Instant activation",
        },
        close: "Close",
        completed: "Payment completed",
        creatingPayment: "Creating payment QR code...",
        payByQr: "Scan to pay",
        paymentUnavailable: {
          description: "The payment QR code is unavailable. Please try again.",
          title: "Payment QR code unavailable",
        },
        price: "Price",
        retry: "Retry",
        scanPrompt: "Scan with a mobile payment app to complete payment",
        secure: {
          description: "Payment data is used for this order only.",
          title: "Secure checkout",
        },
        selectedPlan: "Selected plan",
      },
      dialogs: {
        close: "Close",
        redemptionTitle: "Membership redemption",
        tokenDetailsTitle: "Token details",
        tokenPurchaseTitle: "Buy tokens",
      },
    },
  },
} as const;
