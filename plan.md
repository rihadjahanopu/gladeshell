# 🚀 Gladeshell UX Enhancement Roadmap & Future Feature Plan

এই ডকুমেন্টে `gladeshell`-কে আরও আধুনিক, দ্রুত এবং প্রিমিয়াম (Luxury Level UX) করার ভবিষ্যৎ প্ল্যান এবং ফীচার রোডম্যাপ লিপিবদ্ধ করা হয়েছে।

---

## 🌟 Top 5 UX Enhancements

### 1. 🧹 Transient / Compact Prompt Mode (পরিচ্ছন্ন ইতিহাস মোড)
- **কনসেপ্ট:** আপনি যখন টার্মিনালে কোনো কমান্ড লিখে `Enter` প্রেস করবেন, তখন পূর্ববর্তী লাইনের মাল্টি-লাইন বা বড় প্রম্পট বারটি অটোমেটিক একটি মিনিমালিস্ট ১-লাইনের প্রম্পটে (`❯ git status`) রূপান্তর হয়ে যাবে।
- **সুবিধা:** টার্মিনাল হিস্ট্রি স্ক্রোল করার সময় অপ্রয়োজনীয় স্পেস নষ্ট হবে না এবং আউটপুট দেখতে অনেক ক্লিন লাগবে।

---

### 2. 🛠️ Smart Toolchain & Project Environment Detector
- **কনসেপ্ট:** কোনো প্রজেক্ট ফোল্ডারে ঢুকলে (`cd`), প্রম্পট অটো-ডিটেক্ট করে টুলচেইন ভার্সন দেখাবে:
  - **Node.js প্রজেক্ট** (`package.json`): `⬢ v20.11`
  - **Rust প্রজেক্ট** (`Cargo.toml`): `🦀 v1.76`
  - **Python প্রজেক্ট** (`requirements.txt` / `pyproject.toml`): `🐍 v3.11`
  - **Docker প্রজেক্ট** (`Dockerfile`): `🐳 Docker`

---

### 3. 🔔 Long-Running Command Notification (ডেস্কটপ নোটিফিকেশন)
- **কনসেপ্ট:** কোনো দীর্ঘায়িত প্রসেস বা বিল্ড (যেমন: `cargo build`, `npm install`, `docker build`) যদি ১০ সেকেন্ডের বেশি সময় নেয়, তবে কমান্ডটি শেষ হওয়া মাত্রই ডেস্কটপে নোটিফিকেশন (OSC 99 / System Toast / Sound Bell) দেবে।
- **সুবিধা:** বিল্ড চলাকালীন ব্রাউজার বা অন্য ট্যাবে কাজ করলেও কাজ শেষ হওয়ার সাথে সাথে জানতে পারবেন।

---

### 4. ⚡ Smart Frecent Directory Jumper (`z` Shortcut)
- **কনসেপ্ট:** ব্যবহারকারীর ঘনঘন যাওয়ার ফোল্ডারগুলোর ফ্রিকোয়েন্সি ও রিসেন্সি (`Frecent`) হিস্ট্রি ট্র্যাক করে কাজ করবে। যেমন: `z glade` লিখলেই সরাসরি অতি দ্রুত প্রজেক্ট ফোল্ডারে জ্যাম্প করবে।

---

### 5. 🎨 Interactive Theme Switcher TUI with Live Preview
- **কনসেপ্ট:** `gladeshell theme` রান করলে একটি লাইভ TUI সিলেক্টর ওপেন হবে। **Up / Down Arrow** চাপলেই সরাসরি আপনার টার্মিনালে ৫৫+ টিমের লাইভ প্রিভিউ দেখা যাবে এবং `Enter` চেপে থিম সেভ করা যাবে।

---

## ⚡ Recent Latency & Startup Optimizations (Completed)
- [x] **Zero Subshell Init Loading:** `.zshrc` / `.bashrc` লোড হওয়ার সময় `$(gladeshell socket-path)` এর মত সাবশেল ইনভোকেশন তুলে দেওয়া হয়েছে, যা টার্মিনাল ট্যাব খোলার সময় ব্লাঙ্ক/ব্ল্যাক স্ক্রিনের ল্যাগ সম্পূর্ণ দূর করেছে।
- [x] **Zero-Flicker Prompt Pre-rendering:** প্রম্পট প্রথম ফ্রেমে স্ক্রিনে আসার আগেই প্রি-রেন্ডার করে সেট করা হয়েছে।
