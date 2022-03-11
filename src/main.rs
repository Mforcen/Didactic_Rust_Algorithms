use std::{fmt, ops::Shr, fs, io::Read};

struct Sha256 {
	state: [u32; 8],
	k: [u32; 64]
}

impl fmt::Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x?}{:x?}{:x?}{:x?}{:x?}{:x?}{:x?}{:x?}",
			self.state[0],self.state[1],self.state[2],self.state[3],
			self.state[4],self.state[5],self.state[6],self.state[7]
		)
    }
}

impl Sha256 {
	pub fn new() -> Sha256 {
		Sha256 {
			state: [
				0x6a09e667,
				0xbb67ae85,
				0x3c6ef372,
				0xa54ff53a,
				0x510e527f,
				0x9b05688c,
				0x1f83d9ab,
				0x5be0cd19
			],
			k: [
				0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
				0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
				0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
				0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
				0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
				0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
				0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
				0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
			]
		}
	}

	pub fn process_block(&mut self, data: &[u8]) {
		let mut w = [0u32; 64];
		for (i, &d) in data.iter().enumerate() {
			let byte = i % 4;
			let word = i / 4;
			w[word] |= (d as u32) << ((8*(3-byte)) as u32);
		}
		for i in 16..64 {
			let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ w[i-15].shr(3);
			let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ w[i-2].shr(10);
			w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
		}
		let mut a = self.state[0];
		let mut b = self.state[1];
		let mut c = self.state[2];
		let mut d = self.state[3];
		let mut e = self.state[4];
		let mut f = self.state[5];
		let mut g = self.state[6];
		let mut h = self.state[7];

		for i in 0..64 {
			let ch = (e & f) ^((!e) & g);
			let maj = (a & b)^(a & c)^(b & c);
			let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
			let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
			let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(self.k[i]).wrapping_add(w[i]);
			let t2 = s0.wrapping_add(maj);

			h = g;
			g = f;
			f = e;
			e = d.wrapping_add(t1);
			d = c;
			c = b;
			b = a;
			a = t1.wrapping_add(t2);
		}
		
		self.state[0] = self.state[0].wrapping_add(a);
		self.state[1] = self.state[1].wrapping_add(b);
		self.state[2] = self.state[2].wrapping_add(c);
		self.state[3] = self.state[3].wrapping_add(d);
		self.state[4] = self.state[4].wrapping_add(e);
		self.state[5] = self.state[5].wrapping_add(f);
		self.state[6] = self.state[6].wrapping_add(g);
		self.state[7] = self.state[7].wrapping_add(h);

	}

	pub fn hash(&mut self, v: &mut Vec<u8>) {
		let size= (v.len()*8) as u64;
		v.push(0x80);
		while (v.len()%64) < 56 {
			v.push(0x00);
		}
		let mut s_idx = 0;
		while s_idx < 8 {
			let byte = ((size >> (8*(7-s_idx))) & 0xffu64 ) as u8;
			s_idx += 1;
			v.push(byte);
		}

		for i in 0..(v.len()/64) {
			self.process_block(&v[i*64..(i+1)*64]);
		}
	}
}

fn main() {
	let mut th = Sha256::new();
	let mut file = fs::File::open("foo").unwrap();
	let mut v = Vec::<u8>::new();
	file.read_to_end(&mut v).unwrap();
	th.hash(&mut v);

	println!("{}", th);
}
