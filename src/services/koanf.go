//  Created by silicon-ninja on 13/11/2022
//  Copyright © 2022 Srikanth K. All rights reserved.

package services

import "github.com/knadh/koanf"

var k = koanf.New(".")

// Koanf returns the koanf instance.
func Koanf() *koanf.Koanf {
	return k
}
