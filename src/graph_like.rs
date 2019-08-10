pub trait GraphLike {
    /// Whether there is an edge between vertices n and m
    fn get_edge(&self, n: usize, m: usize) -> bool;

    /// The number of vertices this graph has
    fn num_verts(&self) -> usize;

        //true means connected, false means disconnected
    fn has_k3(&self, col: bool) -> bool {
        if self.num_verts() < 3 {
            return false;
        }
        for i in 0..self.num_verts() - 2 {
            for j in (i + 1)..self.num_verts() - 1 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..self.num_verts() {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k3r(&self, col: bool) -> bool {
        if self.num_verts() < 3 {
            return false;
        }
        for i in 0..=self.num_verts() - 3 {
            for j in (i + 1)..=self.num_verts() - 2 {
                if self.get_edge(i, j) == col
                    && (self.get_edge(i, self.num_verts() - 1) == col)
                    && (self.get_edge(j, self.num_verts() - 1) == col)
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn has_k4(&self, col: bool) -> bool {
        if self.num_verts() < 4 {
            return false;
        }
        for i in 0..self.num_verts() - 3 {
            for j in (i + 1)..self.num_verts() - 2 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..self.num_verts() - 1 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..self.num_verts() {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k4r(&self, col: bool) -> bool {
        if self.num_verts() < 4 {
            return false;
        }
        for i in 0..=self.num_verts() - 4 {
            for j in (i + 1)..=self.num_verts() - 3 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..=self.num_verts() - 2 {
                        if (self.get_edge(i, k) == col)
                            && (self.get_edge(j, k) == col)
                            && (self.get_edge(i, self.num_verts() - 1) == col)
                            && (self.get_edge(j, self.num_verts() - 1) == col)
                            && (self.get_edge(k, self.num_verts() - 1) == col)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k5r(&self, col: bool) -> bool {
        if self.num_verts() < 5 {
            return false;
        }
        for i in 0..=self.num_verts() - 5 {
            for j in (i + 1)..=self.num_verts() - 4 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..=self.num_verts() - 3 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..=self.num_verts() - 2 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                    && (self.get_edge(i, self.num_verts() - 1) == col)
                                    && (self.get_edge(j, self.num_verts() - 1) == col)
                                    && (self.get_edge(k, self.num_verts() - 1) == col)
                                    && (self.get_edge(l, self.num_verts() - 1) == col)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k5(&self, col: bool) -> bool {
        if self.num_verts() < 5 {
            return false;
        }
        for i in 0..self.num_verts() - 4 {
            for j in (i + 1)..self.num_verts() - 3 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..self.num_verts() - 2 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..self.num_verts() - 1 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    for m in (l + 1)..self.num_verts() {
                                        if (self.get_edge(i, m) == col)
                                            && (self.get_edge(j, m) == col)
                                            && (self.get_edge(k, m) == col)
                                            && (self.get_edge(l, m) == col)
                                        {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k6(&self, col: bool) -> bool {
        if self.num_verts() < 6 {
            return false;
        }
        for i in 0..self.num_verts() - 5 {
            for j in (i + 1)..self.num_verts() - 4 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..self.num_verts() - 3 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..self.num_verts() - 2 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    for m in (l + 1)..self.num_verts() - 1 {
                                        if (self.get_edge(i, m) == col)
                                            && (self.get_edge(j, m) == col)
                                            && (self.get_edge(k, m) == col)
                                            && (self.get_edge(l, m) == col)
                                        {
                                            for n in (m + 1)..self.num_verts() {
                                                if (self.get_edge(i, n) == col)
                                                    && (self.get_edge(j, n) == col)
                                                    && (self.get_edge(k, n) == col)
                                                    && (self.get_edge(l, n) == col)
                                                    && (self.get_edge(m, n) == col)
                                                {
                                                    return true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k6r(&self, col: bool) -> bool {
        if self.num_verts() < 6 {
            return false;
        }
        for i in 0..=self.num_verts() - 6 {
            for j in (i + 1)..=self.num_verts() - 5 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..=self.num_verts() - 4 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..=self.num_verts() - 3 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    for m in (l + 1)..=self.num_verts() - 2 {
                                        if (self.get_edge(i, m) == col)
                                            && (self.get_edge(j, m) == col)
                                            && (self.get_edge(k, m) == col)
                                            && (self.get_edge(l, m) == col)
                                            && (self.get_edge(i, self.num_verts() - 1) == col)
                                            && (self.get_edge(j, self.num_verts() - 1) == col)
                                            && (self.get_edge(k, self.num_verts() - 1) == col)
                                            && (self.get_edge(l, self.num_verts() - 1) == col)
                                            && (self.get_edge(m, self.num_verts() - 1) == col)
                                        {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k7(&self, col: bool) -> bool {
        if self.num_verts() < 7 {
            return false;
        }
        for i in 0..self.num_verts() - 6 {
            for j in (i + 1)..self.num_verts() - 5 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..self.num_verts() - 4 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..self.num_verts() - 3 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    for m in (l + 1)..self.num_verts() - 2 {
                                        if (self.get_edge(i, m) == col)
                                            && (self.get_edge(j, m) == col)
                                            && (self.get_edge(k, m) == col)
                                            && (self.get_edge(l, m) == col)
                                        {
                                            for n in (m + 1)..self.num_verts() - 1 {
                                                if (self.get_edge(i, n) == col)
                                                    && (self.get_edge(j, n) == col)
                                                    && (self.get_edge(k, n) == col)
                                                    && (self.get_edge(l, n) == col)
                                                    && (self.get_edge(m, n) == col)
                                                {
                                                    for o in (n + 1)..self.num_verts() {
                                                        if (self.get_edge(i, o) == col)
                                                            && (self.get_edge(j, o) == col)
                                                            && (self.get_edge(k, o) == col)
                                                            && (self.get_edge(l, o) == col)
                                                            && (self.get_edge(m, o) == col)
                                                            && (self.get_edge(n, o) == col)
                                                        {
                                                            return true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_k7r(&self, col: bool) -> bool {
        if self.num_verts() < 7 {
            return false;
        }
        for i in 0..=self.num_verts() - 7 {
            for j in (i + 1)..=self.num_verts() - 6 {
                if self.get_edge(i, j) == col {
                    for k in (j + 1)..=self.num_verts() - 5 {
                        if (self.get_edge(i, k) == col) && (self.get_edge(j, k) == col) {
                            for l in (k + 1)..=self.num_verts() - 4 {
                                if (self.get_edge(i, l) == col)
                                    && (self.get_edge(j, l) == col)
                                    && (self.get_edge(k, l) == col)
                                {
                                    for m in (l + 1)..=self.num_verts() - 3 {
                                        if (self.get_edge(i, m) == col)
                                            && (self.get_edge(j, m) == col)
                                            && (self.get_edge(k, m) == col)
                                            && (self.get_edge(l, m) == col)
                                        {
                                            for n in (m + 1)..self.num_verts() - 2 {
                                                if (self.get_edge(i, n) == col)
                                                    && (self.get_edge(j, n) == col)
                                                    && (self.get_edge(k, n) == col)
                                                    && (self.get_edge(l, n) == col)
                                                    && (self.get_edge(m, n) == col)
                                                    && (self.get_edge(i, self.num_verts() - 1) == col)
                                                    && (self.get_edge(j, self.num_verts() - 1) == col)
                                                    && (self.get_edge(k, self.num_verts() - 1) == col)
                                                    && (self.get_edge(l, self.num_verts() - 1) == col)
                                                    && (self.get_edge(m, self.num_verts() - 1) == col)
                                                    && (self.get_edge(n, self.num_verts() - 1) == col)
                                                {
                                                    return true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn has_kns(&self, n: u32, m: u32) -> bool {
        let kn = match n {
            3 => self.has_k3r(true),
            4 => self.has_k4r(true),
            5 => self.has_k5r(true),
            6 => self.has_k6r(true),
            7 => self.has_k7r(true),
            _ => self.has_k3(true),
        };
        if !kn {
            let km = match m {
                3 => self.has_k3r(false),
                4 => self.has_k4r(false),
                5 => self.has_k5r(false),
                6 => self.has_k6r(false),
                7 => self.has_k7r(false),
                _ => self.has_k3(false),
            };
            if km {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }

}
