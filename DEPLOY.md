# 🚀 วิธีอัพโปรเจกต์ขึ้น GitHub

## ขั้นตอนที่ 1: สร้าง Repository บน GitHub

1. ไปที่ https://github.com/new
2. ตั้งชื่อ repository: `github-profile-rust`
3. เลือก Public หรือ Private ตามต้องการ
4. **อย่า** เลือก "Initialize this repository with a README"
5. คลิก "Create repository"

## ขั้นตอนที่ 2: เตรียมไฟล์ก่อน Push

### 2.1 สร้างไฟล์ config.toml

```bash
# คัดลอกจาก example
Copy-Item config.example.toml config.toml

# แก้ไขไฟล์ config.toml ให้ใส่ username ของคุณ
notepad config.toml
```

แก้ไขบรรทัดแรกเป็น GitHub username ของคุณ:
```toml
github_username = "your-actual-username"
```

### 2.2 ทดสอบว่าใช้งานได้

```bash
cargo run -- --config config.toml
```

## ขั้นตอนที่ 3: Push ขึ้น GitHub

```bash
# เริ่มต้น Git repository
git init

# เพิ่มไฟล์ทั้งหมด
git add .

# Commit
git commit -m "🎉 Initial commit: GitHub profile generator in Rust"

# เชื่อมต่อกับ GitHub (แทน YOUR-USERNAME ด้วย username ของคุณ)
git remote add origin https://github.com/YOUR-USERNAME/github-profile-rust.git

# เปลี่ยน branch เป็น main
git branch -M main

# Push ขึ้น GitHub
git push -u origin main
```

## ขั้นตอนที่ 4: ตั้งค่า GitHub Actions (ถ้าต้องการอัปเดตอัตโนมัติ)

1. ไปที่ repository บน GitHub
2. ไปที่ Settings → Actions → General
3. เลื่อนลงไปที่ "Workflow permissions"
4. เลือก "Read and write permissions"
5. คลิก Save

## ขั้นตอนที่ 5: สร้าง Profile Repository (สำหรับ GitHub Profile)

ถ้าคุณต้องการให้ README แสดงบน GitHub profile ของคุณ:

1. สร้าง repository ใหม่ชื่อเดียวกับ username ของคุณ
   - เช่น ถ้า username คุณคือ `johndoe` ให้สร้าง repo ชื่อ `johndoe`
2. เลือก Public
3. เลือก "Add a README file"
4. คัดลอก README.md ที่ generate ได้ไปวางใน repository นั้น

## 🎯 สรุปคำสั่งทั้งหมด

```bash
# 1. สร้าง config.toml
Copy-Item config.example.toml config.toml

# 2. แก้ไข config.toml (ใส่ username ของคุณ)
notepad config.toml

# 3. ทดสอบ
cargo run -- --config config.toml

# 4. Git commands
git init
git add .
git commit -m "🎉 Initial commit: GitHub profile generator in Rust"
git remote add origin https://github.com/YOUR-USERNAME/github-profile-rust.git
git branch -M main
git push -u origin main
```

## 💡 Tips

- ไฟล์ `config.toml` จะไม่ถูก push ขึ้น GitHub (อยู่ใน .gitignore)
- คุณสามารถแก้ไข `config.example.toml` เป็นตัวอย่างที่ดีสำหรับคนอื่น
- GitHub Actions จะรันทุกวันเวลาเที่ยงคืน UTC โดยอัตโนมัติ
